use llvm_ir::Function;
use egg::{EGraph, Id};
use llvm_ir::instruction::BinaryOp;
use crate::{Lang, analysis};

type LangEgraph = EGraph<Lang, analysis::ConstFold>;

pub struct FunctionEgraph {
    egraph: LangEgraph,
    root: Id,
    parameters: Vec<llvm_ir::function::Parameter>,
    return_type: llvm_ir::TypeRef,
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

// pub fn convert_to_egraph(func: Function) -> FunctionEgraph {
//     let mut graph = LangEgraph::default();
//     for block in func.basic_blocks {
//         match block.term {
//             llvm_ir::Terminator::Ret(llvm_ir::terminator::Ret{ return_operand, debugloc: _ }) => match return_operand {
//                 Some(op) => match op {
//                     llvm_ir::operand::Operand::ConstantOperand(const_ref) => { Lang::from(const_ref.as_ref()); },
//                     _ => unimplemented!(),
//                 }
//                 None => {}
//             }
//             _ => {}
//         }
//     }
//     unimplemented!();
// }

fn parse_basic_block(block: &llvm_ir::BasicBlock, egraph: &mut LangEgraph) -> Result<(), &'static str> {
    use llvm_ir::Instruction as Instr;
    for instr in block.instrs.iter() {
        let node = match instr {
            Instr::Add(instr) => Lang::Add([
                        egraph.add(instr.get_operand0().into()),
                        egraph.add(instr.get_operand1().into())
            ]),
            Instr::Sub(instr) => Lang::Sub([
                        egraph.add(instr.get_operand0().into()),
                        egraph.add(instr.get_operand1().into())
            ]),
            _ => todo!(),
        };
        let name = match instr {
            Instr::Add(instr) => (&instr.dest).into(),
            Instr::Sub(instr) => (&instr.dest).into(),
            _ => todo!(),
        };
        egraph.add(node);
        egraph.add(name);
    }

    Ok(())
}
