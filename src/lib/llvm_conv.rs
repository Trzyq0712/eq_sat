use crate::{analysis, Lang};
use egg::{EGraph, Id};
use llvm_ir::Function;

type LangEgraph = EGraph<Lang, analysis::ConstFold>;

#[derive(Debug, Clone)]
pub struct FunctionEgraph {
    egraph: LangEgraph,
    root: Id,
    function: Function,
}

impl From<&llvm_ir::Constant> for crate::Lang {
    fn from(value: &llvm_ir::Constant) -> Self {
        match *value {
            llvm_ir::Constant::Int { bits: _, value } => Lang::Num(value as i32),
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
            let ret_id = egraph
                .lookup::<Lang>(ret.return_operand.as_ref().unwrap().into())
                .unwrap();
            root = Some(ret_id);
        }
    }
    let root = root.unwrap();
    Ok(FunctionEgraph {
        egraph,
        root,
        function,
    })
}

fn parse_basic_block(block: &llvm_ir::BasicBlock, egraph: &mut LangEgraph) -> Result<(), String> {
    use llvm_ir::Instruction as Instr;
    let add_binop = |instr: &dyn llvm_ir::instruction::BinaryOp, eg: &mut LangEgraph| {
        [
            eg.add(instr.get_operand0().into()),
            eg.add(instr.get_operand1().into()),
        ]
    };
    for instr in block.instrs.iter() {
        let node = match instr {
            Instr::Add(instr) => Lang::Add(add_binop(instr, egraph)),
            Instr::Sub(instr) => Lang::Sub(add_binop(instr, egraph)),
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

fn drop_allocations(func: Function) -> Function {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;
    // #[test]
    // fn simple_func_to_egraph() {
    //     use llvm_ir::*;
    //     let int_type = types::Type::IntegerType { bits: 32 };
    //     let x_param = name::Name::Name(Box::new("x".to_owned()));
    //     let y_param = name::Name::Name(Box::new("y".to_owned()));
    //     let func = Function {
    //         name: "Test func".into(),
    //         parameters: vec![
    //             function::Parameter {
    //                 name: x_param.clone(),
    //                 ty: &int_type,
    //                 ..Default::default() },
    //             function::Parameter {
    //                 name: y_param.clone()
    //                 ty: &int_type,
    //                 ..Default::default() },
    //         ],
    //         return_type: &int_type,
    //         basic_blocks: vec![BasicBlock {
    //             name: name::Name::Number(0),
    //             instrs: vec![instruction::Instruction::Add(instruction::Add {
    //                 operand0: Operand::LocalOperand { name: x_param, ty: &int_type },
    //                 operand1: Operand::LocalOperand { name: y_param, ty: &int_type },
    //                 dest: name::Name::Number(1),
    //                 ..Default::default(),
    //             })],
    //             term: terminator::Terminator::Ret(terminator::Ret {
    //                 return_operand: Some(Operand::LocalOperand { name: name::Name::Number(1), ty: &int_type }),
    //                 ..Default::default(),
    //             }),
    //         }],
    //         ..Default::default()
    //     };
    //     let func_egraph = convert_to_egraph(func);
    //     dbg!(&func_egraph);
    //     panic!();
    // }
}
