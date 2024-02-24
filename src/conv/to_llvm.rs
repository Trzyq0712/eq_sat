type Id = String;

enum LLVMType {
    I64,
    I1,
    Void,
}

struct LLVMFunction {
    name: String,
    params: Vec<(LLVMType, Id)>,
    ret_ty: LLVMType,
    entry: LLVMBlock,
    blocks: Vec<(Id, LLVMBlock)>,
}

struct LLVMBlock {
    instrs: Vec<LLVMInstr>,
    term: LLVMTerm,
}

enum LLVMInstr {
    Add((Id, LLVMType, LLVMValue, LLVMValue)),
    Mul((Id, LLVMType, LLVMValue, LLVMValue)),
    ICmp((Id, crate::lang::Cond, LLVMType, LLVMValue, LLVMValue)),
}

enum LLVMTerm {
    Ret((LLVMType, LLVMValue)),
    Br(Id),
    CBr((LLVMValue, Id, Id)),
}

enum LLVMValue {
    I64(i64),
    Bool(bool),
    Id(Id),
}

impl std::fmt::Display for LLVMType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LLVMType::I64 => write!(f, "i64"),
            LLVMType::I1 => write!(f, "i1"),
            LLVMType::Void => write!(f, "void"),
        }
    }
}

impl std::fmt::Display for LLVMValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LLVMValue::I64(i) => write!(f, "{}", i),
            LLVMValue::Bool(b) => write!(f, "{}", b),
            LLVMValue::Id(id) => write!(f, "%{}", id),
        }
    }
}

impl std::fmt::Display for LLVMInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LLVMInstr::Add((dst, ty, lhs, rhs)) => write!(f, "\t%{dst} = add {ty} {lhs}, {rhs}"),
            LLVMInstr::Mul((dst, ty, lhs, rhs)) => write!(f, "\t%{dst} = mul {ty} {lhs}, {rhs}"),
            LLVMInstr::ICmp((dst, cond, ty, lhs, rhs)) => {
                use crate::lang::Cond;
                let cond = match cond {
                    Cond::Eq => "eq",
                    Cond::Neq => "ne",
                    Cond::Lt => "slt",
                    Cond::Leq => "sle",
                    Cond::Gt => "sgt",
                    Cond::Geq => "sge",
                };
                write!(f, "\t%{dst} = icmp {cond} {ty} {lhs}, {rhs}")
            }
        }
    }
}

impl std::fmt::Display for LLVMTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LLVMTerm::Ret((ty, val)) => write!(f, "\tret {ty} {val}"),
            LLVMTerm::Br(block) => write!(f, "\tbr label %{}", block),
            LLVMTerm::CBr((cond, if_true, if_false)) => {
                write!(f, "\tbr i1 {}, label %{}, label %{}", cond, if_true, if_false)
            }
        }
    }
}

impl std::fmt::Display for LLVMBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for instr in &self.instrs {
            writeln!(f, "{}", instr)?;
        }
        write!(f, "{}", self.term)
    }
}

impl std::fmt::Display for LLVMFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "define {} @{}({}) {{", self.ret_ty, self.name, self.params.iter().map(|(ty, name)| format!("{ty} %{name}")).collect::<Vec<_>>().join(", "))?;
        writeln!(f, "{}", self.entry)?;
        for (id, block) in &self.blocks {
            writeln!(f, "{}:", id)?;
            writeln!(f, "{}", block)?;
        }
        writeln!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_function() {
        let f = LLVMFunction {
            name: "add".into(),
            params: vec![(LLVMType::I64, "a".into()), (LLVMType::I64, "b".into())],
            ret_ty: LLVMType::I64,
            entry: LLVMBlock {
                instrs: vec![LLVMInstr::Add(("res".into(), LLVMType::I64, LLVMValue::Id("a".into()), LLVMValue::Id("b".into())))],
                term: LLVMTerm::Ret((LLVMType::I64, LLVMValue::Id("res".into()))),
            },
            blocks: vec![],
        };

        println!("{}", f);
    }
}
