use egg::RecExpr;

use super::cfg::Cfg;
use crate::{lang, Lang};

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Operand {
    Constant(i64),
    Variable(String),
}

struct Context {
    to_id: HashMap<Operand, egg::Id>,
    alloc_ctr: u64,
    ptr_state: Vec<HashMap<egg::Id, egg::Id>>, // block_id -> ptr -> witness
    block_cond: Vec<egg::Id>,
    ret: Option<egg::Id>,
    cfg: Cfg,
}

impl Context {
    fn new(bblocks: &[llvm_ir::BasicBlock], to_id: HashMap<Operand, egg::Id>) -> Self {
        Self {
            to_id,
            alloc_ctr: 0,
            ptr_state: vec![HashMap::new(); bblocks.len()],
            block_cond: vec![Default::default(); bblocks.len()],
            ret: None,
            cfg: Cfg::new(bblocks),
        }
    }

    fn get_or_add_id(&mut self, egraph: &mut RecExpr<Lang>, operand: &Operand) -> egg::Id {
        if let Some(id) = self.to_id.get(operand) {
            return *id;
        }

        let id = match operand {
            Operand::Constant(value) => egraph.add(Lang::I64(*value)),
            Operand::Variable(name) => panic!("Op {:?}: Variable {} not found", operand, name),
        };

        self.to_id.insert(operand.clone(), id);
        id
    }
}

pub fn from_module(_module: &llvm_ir::Module) {
    todo!()
}

pub fn parse_function(function: &llvm_ir::Function) -> (RecExpr<Lang>, egg::Id) {
    let mut egraph = RecExpr::default();
    let mut name_to_id: HashMap<Operand, egg::Id> = HashMap::new();

    for param in &function.parameters {
        let id = egraph.add((&param.name).into());
        name_to_id.insert((&param.name).into(), id);
    }

    let bblocks = &function.basic_blocks;
    let mut ctx = Context::new(bblocks, name_to_id);

    let block_order = ctx.cfg.topo_order();
    for block_id in block_order {
        parse_bblock(&mut ctx, &mut egraph, &bblocks[block_id]);
    }

    (egraph, ctx.ret.expect("No return value"))
}

fn parse_bblock(ctx: &mut Context, egraph: &mut RecExpr<Lang>, bblock: &llvm_ir::BasicBlock) {
    let block_id = ctx.cfg.id_of(&bblock.name);
    let preds = ctx.cfg.preds(&bblock.name).to_vec();
    match preds.as_slice() {
        &[] => {
            let id = egraph.add(Lang::I1(true));
            ctx.block_cond[block_id] = id;
        }
        preds => {
            let pred_conds: Vec<_> = preds
                .iter()
                .map(|&pred| (pred, ctx.block_cond[pred]))
                .collect();
            // Ptrs shared by all predecessors
            let shared_ptrs = preds.iter().skip(1).fold(
                ctx.ptr_state[preds[0]].keys().collect::<Vec<_>>(),
                |acc, &pred| {
                    acc.into_iter()
                        .filter(|&ptr| ctx.ptr_state[pred].contains_key(ptr))
                        .collect()
                },
            );

            let mut ptr_state = HashMap::new();
            for ptr in shared_ptrs {
                // (block_id, block_cond)
                let mut curr_witness = *ctx.ptr_state[preds[0]].get(ptr).unwrap();
                for (pred_id, pred_cond) in &pred_conds[1..] {
                    let witness = *ctx.ptr_state[*pred_id].get(ptr).unwrap();
                    let id = egraph.add(Lang::Phi([*pred_cond, witness, curr_witness]));
                    curr_witness = id;
                }
                ptr_state.insert(*ptr, curr_witness);
            }
            ctx.ptr_state[block_id] = ptr_state;

            let get_pred_cond = |ctx: &mut Context, egraph: &mut RecExpr<Lang>, pred: usize| {
                let term = ctx.cfg.blocks[pred].term.clone();
                match term {
                    llvm_ir::Terminator::CondBr(cond_br) => {
                        let cond = ctx.get_or_add_id(egraph, &(&cond_br.condition).into());
                        let cond = if cond_br.false_dest == bblock.name {
                            egraph.add(Lang::Not(cond))
                        } else {
                            cond
                        };
                        egraph.add(Lang::And([cond, ctx.block_cond[pred]]))
                    }
                    llvm_ir::Terminator::Br(_) => ctx.block_cond[pred],
                    _ => panic!("Expected branch"),
                }
            };

            let mut block_cond = get_pred_cond(ctx, egraph, preds[0]);
            for pred in &preds[1..] {
                let pred_cond = get_pred_cond(ctx, egraph, *pred);
                block_cond = egraph.add(Lang::Or([pred_cond, block_cond]));
            }
            ctx.block_cond[block_id] = block_cond;
        }
    }

    for instruction in &bblock.instrs {
        parse_instruction(ctx, egraph, &bblock.name, instruction);
    }
    ctx.ret = match &bblock.term {
        llvm_ir::Terminator::Ret(ret) => {
            let op = ctx.get_or_add_id(
                egraph,
                &(ret.return_operand.as_ref().expect("Void function")).into(),
            );
            Some(op)
        }
        _ => None,
    };
}

fn parse_instruction(
    ctx: &mut Context,
    egraph: &mut RecExpr<Lang>,
    curr_block: &llvm_ir::Name,
    instr: &llvm_ir::Instruction,
) {
    let block_id = ctx.cfg.id_of(curr_block);
    match instr {
        llvm_ir::Instruction::Alloca(alloca) => {
            let witness = egraph.add(Lang::Alloca(ctx.alloc_ctr));
            ctx.alloc_ctr += 1;

            let ptr = egraph.add(Lang::Ptr(witness));
            ctx.to_id.insert((&alloca.dest).into(), ptr);
            ctx.ptr_state[block_id].insert(ptr, witness);
        }
        llvm_ir::Instruction::Load(load) => {
            let ptr = ctx.get_or_add_id(egraph, &(&load.address).into());
            let witness = *ctx.ptr_state[block_id]
                .get(&ptr)
                .expect("Pointer witness not found");
            let id = egraph.add(Lang::Load([witness, ptr]));
            ctx.to_id.insert((&load.dest).into(), id);
            // Loads do not affect state, so no need to update ptr_state
        }
        llvm_ir::Instruction::Store(store) => {
            let ptr = ctx.get_or_add_id(egraph, &(&store.address).into());
            let value = ctx.get_or_add_id(egraph, &(&store.value).into());
            let witness = *ctx.ptr_state[block_id]
                .get(&ptr)
                .expect("Pointer witness not found");
            let id = egraph.add(Lang::Store([value, witness, ptr]));
            ctx.ptr_state[block_id].insert(ptr, id); // Now this load is the witness
        }
        llvm_ir::Instruction::ICmp(icmp) => {
            let op0 = ctx.get_or_add_id(egraph, &(&icmp.operand0).into());
            let op1 = ctx.get_or_add_id(egraph, &(&icmp.operand1).into());
            let cond = match icmp.predicate {
                llvm_ir::IntPredicate::EQ => lang::Cond::Eq,
                llvm_ir::IntPredicate::NE => lang::Cond::Neq,
                llvm_ir::IntPredicate::SGT => lang::Cond::Gt,
                llvm_ir::IntPredicate::SGE => lang::Cond::Geq,
                llvm_ir::IntPredicate::SLT => lang::Cond::Lt,
                llvm_ir::IntPredicate::SLE => lang::Cond::Leq,
                _ => todo!("unimplemented"),
            };
            let id = egraph.add(Lang::ICmp(cond, [op0, op1]));
            ctx.to_id.insert((&icmp.dest).into(), id);
        }
        llvm_ir::Instruction::Add(add) => {
            let op0 = ctx.get_or_add_id(egraph, &(&add.operand0).into());
            let op1 = ctx.get_or_add_id(egraph, &(&add.operand1).into());
            let id = egraph.add(Lang::Add([op0, op1]));
            ctx.to_id.insert((&add.dest).into(), id);
        }
        llvm_ir::Instruction::Sub(sub) => {
            let op0 = ctx.get_or_add_id(egraph, &(&sub.operand0).into());
            let op1 = ctx.get_or_add_id(egraph, &(&sub.operand1).into());
            let id = egraph.add(Lang::Sub([op0, op1]));
            ctx.to_id.insert((&sub.dest).into(), id);
        }
        llvm_ir::Instruction::Mul(mul) => {
            let op0 = ctx.get_or_add_id(egraph, &(&mul.operand0).into());
            let op1 = ctx.get_or_add_id(egraph, &(&mul.operand1).into());
            let id = egraph.add(Lang::Mul([op0, op1]));
            ctx.to_id.insert((&mul.dest).into(), id);
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
                Operand::Variable(name_to_string(name))
            }
            llvm_ir::Operand::ConstantOperand(cons_ref) => match cons_ref.as_ref() {
                &llvm_ir::Constant::Int { bits: _bits, value } => Operand::Constant(value as i64),
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
        Lang::Var(egg::Symbol::new(symbol))
    }
}
