use llvm_ir::Function;
use egg::{EGraph, Id};
use llvm_ir::instruction::BinaryOp;
use crate::{Lang, analysis};

type LangEgraph = EGraph<Lang, analysis::ConstFold>;

pub struct FunctionEgraph {
    egraph: LangEgraph,
    root: Id,
    function: Function,
}

impl From<&llvm_ir::Constant> for crate::Lang {
    fn from(value: &llvm_ir::Constant) -> Self {
        match *value {
            llvm_ir::Constant::Int{ bits: _, value } => Lang::Num(value as i32),
            _ => unimplemented!(),
        }
    }
}

impl From<&llvm_ir::Operand> for crate::Lang {
    fn from(value: &llvm_ir::Operand) -> Self {
        use llvm_ir::Operand;
        match value {
            Operand::LocalOperand { name, ty: _ } => name.into(),
            Operand::ConstantOperand(const_op) => const_op.as_ref().into(),
            _ => unimplemented!("Meta data operands not implemented"),
        }
    }
}

impl From<&llvm_ir::Name> for crate::Lang {
    fn from(value: &llvm_ir::Name) -> Self {
        match value {
            llvm_ir::name::Name::Name(name) => Lang::Symbol(name.as_ref().into()),
            llvm_ir::name::Name::Number(n) => Lang::Temp(*n),
        }
    }
}

pub fn convert_to_egraph(function: Function) -> Result<FunctionEgraph, String> {
    let mut egraph = LangEgraph::default();
    let mut root = None;
    for block in function.basic_blocks.iter() {
        parse_basic_block(block, &mut egraph)?;
        if let llvm_ir::terminator::Terminator::Ret(ret) = &block.term {
            let ret_id = egraph.lookup::<Lang>(ret.return_operand.as_ref().unwrap().into()).unwrap();
            root = Some(ret_id);
        }
    }
    let root = root.unwrap();
    Ok(FunctionEgraph { egraph, root, function })
}

macro_rules! bininstr_to_node {
    ($instr:expr, $graph:expr) => {
        [
            $graph.add($instr.get_operand0().into()),
            $graph.add($instr.get_operand1().into())
        ]
    };
}

fn parse_basic_block(block: &llvm_ir::BasicBlock, egraph: &mut LangEgraph) -> Result<(), String> {
    use llvm_ir::Instruction as Instr;
    for instr in block.instrs.iter() {
        let node = match instr {
            Instr::Add(instr) => Lang::Add(bininstr_to_node!(instr, egraph)),
            Instr::Sub(instr) => Lang::Sub(bininstr_to_node!(instr, egraph)),
            _ => todo!(),
        };
        let name = match instr {
            Instr::Add(instr) => (&instr.dest).into(),
            Instr::Sub(instr) => (&instr.dest).into(),
            _ => todo!(),
        };
        let node_id = egraph.add(node);
        let name_id = egraph.add(name);
        egraph.union(node_id, name_id);
    }
    Ok(())
}
