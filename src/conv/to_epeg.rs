use super::cfg::Cfg;
use crate::{EGraph, Lang};

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Operand {
    Constant(i64),
    Variable(String),
}

struct Context {
    to_id: HashMap<Operand, egg::Id>,
    alloc_ctr: usize,
    ptr_state: Vec<HashMap<egg::Id, egg::Id>>,
    cfg: Cfg,
}

impl Context {
    fn new(bblocks: &[llvm_ir::BasicBlock], to_id: HashMap<Operand, egg::Id>) -> Self {
        Self {
            to_id,
            alloc_ctr: 0,
            ptr_state: vec![HashMap::new(); bblocks.len()],
            cfg: Cfg::new(bblocks),
        }
    }
    fn to_id(&mut self, egraph: &mut EGraph, operand: &Operand) -> egg::Id {
        if let Some(id) = self.to_id.get(operand) {
            return *id;
        }

        let id = match operand {
            Operand::Constant(value) => egraph.add(Lang::Num(*value)),
            Operand::Variable(name) => panic!("Op {:?}: Variable {} not found", operand, name),
        };

        self.to_id.insert(operand.clone(), id);
        id
    }
}

pub fn from_module(module: &llvm_ir::Module) {
    todo!()
}

pub fn parse_function(function: &llvm_ir::Function) -> EGraph {
    assert!(!function.is_var_arg, "Variable arguments are not supported");

    let mut egraph = EGraph::default();
    let mut name_to_id: HashMap<Operand, egg::Id> = HashMap::new();

    for param in &function.parameters {
        let id = egraph.add((&param.name).into());
        name_to_id.insert((&param.name).into(), id);
    }

    dbg!(&egraph);
    dbg!(&name_to_id);

    let bblocks = &function.basic_blocks;
    let mut ctx = Context::new(bblocks, name_to_id);

    let block_order = ctx.cfg.topo_order();
    for block_id in block_order {
        parse_bblock(&mut ctx, &mut egraph, &bblocks[block_id]);
    }

    egraph
}

fn parse_bblock(ctx: &mut Context, egraph: &mut EGraph, bblock: &llvm_ir::BasicBlock) {
    let &block_id = ctx.cfg.block_name_to_id.get(&bblock.name).unwrap();
    let preds = ctx.cfg.get_preds(&bblock.name);
    match preds {
        &[] => {}
        &[pred] => {
            ctx.ptr_state[block_id] = ctx.ptr_state[pred].clone();
        }
        &[pred1, pred2] => {
            let (dec_point, (src_true, src_false)) = ctx.cfg.get_decision_point(&bblock.name);
            let &dec_block_id = ctx.cfg.block_name_to_id.get(&dec_point).unwrap();
            let dec_block = &ctx.cfg.blocks[dec_block_id];
            let llvm_ir::Terminator::CondBr(cond_br) = &dec_block.term else {
                panic!("Expected conditional branch");
            };
            dbg!(&src_true, &src_false);
            let cond = ctx.to_id(egraph, &(&cond_br.condition).into());
            let &true_id = ctx.cfg.block_name_to_id.get(&src_true).unwrap();
            let &false_id = ctx.cfg.block_name_to_id.get(&src_false).unwrap();
            let common_ptrs = ctx.ptr_state[true_id]
                .keys()
                .filter(|&ptr| ctx.ptr_state[false_id].contains_key(ptr));
            let mut ptr_state = HashMap::new();
            for ptr in common_ptrs {
                let &witness_true = ctx.ptr_state[true_id].get(ptr).unwrap();
                let &witness_false = ctx.ptr_state[false_id].get(ptr).unwrap();
                let id = egraph.add(Lang::Phi([cond, witness_true, witness_false]));
                ptr_state.insert(*ptr, id);
            }
            ctx.ptr_state[block_id] = ptr_state;
        }
        _ => panic!("More than two predecessors"),
    }
    println!("Before {}: {:?}", bblock.name, {
        let mut ptrs: Vec<_> = ctx.ptr_state[block_id].iter().collect();
        ptrs.sort_unstable();
        ptrs
    });
    for instruction in &bblock.instrs {
        parse_instruction(ctx, egraph, &bblock.name, instruction);
    }
    println!("After {}: {:?}", bblock.name, {
        let mut ptrs: Vec<_> = ctx.ptr_state[block_id].iter().collect();
        ptrs.sort_unstable();
        ptrs
    });
}

fn parse_instruction(
    ctx: &mut Context,
    egraph: &mut EGraph,
    curr_block: &llvm_ir::Name,
    instr: &llvm_ir::Instruction,
) {
    let &block_id = ctx.cfg.block_name_to_id.get(curr_block).unwrap();
    match instr {
        llvm_ir::Instruction::Add(add) => {
            let op0 = ctx.to_id(egraph, &(&add.operand0).into());
            let op1 = ctx.to_id(egraph, &(&add.operand1).into());
            let id = egraph.add(Lang::Add([op0, op1]));
            ctx.to_id.insert((&add.dest).into(), id);
        }
        llvm_ir::Instruction::Alloca(alloca) => {
            let witness = egraph.add(Lang::Alloca(ctx.alloc_ctr));
            ctx.alloc_ctr += 1;

            let ptr = egraph.add(Lang::Ptr(witness));
            ctx.to_id.insert((&alloca.dest).into(), ptr);
            ctx.ptr_state[block_id].insert(ptr, witness);
        }
        llvm_ir::Instruction::Load(load) => {
            let ptr = ctx.to_id(egraph, &(&load.address).into());
            let witness = *ctx.ptr_state[block_id]
                .get(&ptr)
                .expect("Pointer witness not found");
            let id = egraph.add(Lang::Load([ptr, witness]));
            ctx.to_id.insert((&load.dest).into(), id);
            // Loads do not affect state, so no need to update ptr_state
        }
        llvm_ir::Instruction::Store(store) => {
            let ptr = ctx.to_id(egraph, &(&store.address).into());
            let value = ctx.to_id(egraph, &(&store.value).into());
            let witness = *ctx.ptr_state[block_id]
                .get(&ptr)
                .expect("Pointer witness not found");
            let id = egraph.add(Lang::Store([value, ptr, witness]));
            ctx.ptr_state[block_id].insert(ptr, id); // Now this load is the witness
        }
        llvm_ir::Instruction::ICmp(icmp) => {
            let op0 = ctx.to_id(egraph, &(&icmp.operand0).into());
            let op1 = ctx.to_id(egraph, &(&icmp.operand1).into());
            let id = egraph.add(Lang::ICmp([op0, op1]));
            ctx.to_id.insert((&icmp.dest).into(), id);
        }
        _ => todo!(),
    }
}

// fn parse_terminator(ctx: &mut Context, egraph: &mut EGraph, term: &llvm_ir::Terminator) {
//     match term {
//         llvm_ir::Terminator::Ret(ret) => {
//             let op = ctx.to_id(egraph, &(&ret.return_operand).into());
//             // let id = egraph.add(Lang::Ret(op));
//         }
//         _ => todo!(),
//     }
// }

// fn parse_operand(context: &Context, operand: &llvm_ir::Operand) {
//     todo!()
// }

impl From<&llvm_ir::Operand> for Operand {
    fn from(operand: &llvm_ir::Operand) -> Self {
        match operand {
            llvm_ir::Operand::LocalOperand { name, ty: _ } => {
                Operand::Variable(name_to_string(&name))
            }
            llvm_ir::Operand::ConstantOperand(cons_ref) => match cons_ref.as_ref() {
                &llvm_ir::Constant::Int { bits, value } => Operand::Constant(value as i64),
                _ => todo!(),
            },
            llvm_ir::Operand::MetadataOperand => todo!(),
        }
    }
}

fn name_to_string(name: &llvm_ir::Name) -> String {
    match name {
        llvm_ir::Name::Name(name) => *name.clone(),
        llvm_ir::Name::Number(number) => number.to_string(),
    }
}

impl From<&llvm_ir::Name> for Operand {
    fn from(name: &llvm_ir::Name) -> Self {
        let varop = name_to_string(name);
        Operand::Variable(varop)
    }
}

impl From<&llvm_ir::Name> for Lang {
    fn from(name: &llvm_ir::Name) -> Self {
        let symbol = name_to_string(name);
        Lang::Symbol(egg::Symbol::new(symbol))
    }
}
