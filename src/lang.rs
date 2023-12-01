use std::fmt::Display;
use std::fmt::Formatter;
use std::sync::atomic::AtomicU64;

use egg::{FromOp, Id, Language, Symbol};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Cond {
    Eq,
    Neq,
    Lt,
    Gt,
    Leq,
    Geq,
}

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum Lang {
    Add([Id; 2]),
    Sub([Id; 2]),
    Mul([Id; 2]),

    BAnd([Id; 2]),
    BOr([Id; 2]),
    BNot(Id),

    And([Id; 2]),
    Or([Id; 2]),
    Not(Id),
    ICmp(Cond, [Id; 2]),

    Phi([Id; 3]),

    Eval([Id; 2]), // sequence, nth of
    Pass(Id),      // returns index of first true in sequence

    Alloca(u64),
    Ptr(Id),
    Load([Id; 2]),  // witness, ptr
    Store([Id; 3]), // val, witness, ptr

    I1(bool),
    I64(i64),
    Var(Symbol),
}

impl Language for Lang {
    fn matches(&self, other: &Self) -> bool {
        use Lang::*;
        match (self, other) {
            (Add(_), Add(_)) => true,
            (Sub(_), Sub(_)) => true,
            (Mul(_), Mul(_)) => true,
            (BAnd(_), BAnd(_)) => true,
            (BOr(_), BOr(_)) => true,
            (BNot(_), BNot(_)) => true,
            (And(_), And(_)) => true,
            (Or(_), Or(_)) => true,
            (Not(_), Not(_)) => true,
            (ICmp(a, _), ICmp(b, _)) => a == b,
            (Phi(_), Phi(_)) => true,
            (Eval(_), Eval(_)) => true,
            (Pass(_), Pass(_)) => true,
            (Ptr(_), Ptr(_)) => true,
            (Load(_), Load(_)) => true,
            (Store(_), Store(_)) => true,
            (I1(a), I1(b)) => a == b,
            (I64(a), I64(b)) => a == b,
            (Var(a), Var(b)) => a == b,
            (Alloca(a), Alloca(b)) => a == b,
            _ => false,
        }
    }

    fn children(&self) -> &[Id] {
        match self {
            Lang::Add(ops) => ops,
            Lang::Sub(ops) => ops,
            Lang::Mul(ops) => ops,
            Lang::BAnd(ops) => ops,
            Lang::BOr(ops) => ops,
            Lang::BNot(op) => std::slice::from_ref(op),

            Lang::And(ops) => ops,
            Lang::Or(ops) => ops,
            Lang::Not(op) => std::slice::from_ref(op),

            Lang::ICmp(_, ops) => ops,
            Lang::Phi(ops) => ops,
            Lang::Eval(ops) => ops,
            Lang::Pass(op) => std::slice::from_ref(op),

            Lang::Alloca(_) => &[],
            Lang::Ptr(op) => std::slice::from_ref(op),
            Lang::Load(ops) => ops,
            Lang::Store(ops) => ops,

            Lang::I1(_) => &[],
            Lang::I64(_) => &[],
            Lang::Var(_) => &[],
        }
    }

    fn children_mut(&mut self) -> &mut [Id] {
        match self {
            Lang::Add(ops) => ops,
            Lang::Sub(ops) => ops,
            Lang::Mul(ops) => ops,
            Lang::BAnd(ops) => ops,
            Lang::BOr(ops) => ops,
            Lang::BNot(op) => std::slice::from_mut(op),

            Lang::And(ops) => ops,
            Lang::Or(ops) => ops,
            Lang::Not(op) => std::slice::from_mut(op),

            Lang::ICmp(_, ops) => ops,
            Lang::Phi(ops) => ops,
            Lang::Eval(ops) => ops,
            Lang::Pass(op) => std::slice::from_mut(op),

            Lang::Alloca(_) => &mut [],
            Lang::Ptr(op) => std::slice::from_mut(op),
            Lang::Load(ops) => ops,
            Lang::Store(ops) => ops,

            Lang::I1(_) => &mut [],
            Lang::I64(_) => &mut [],
            Lang::Var(_) => &mut [],
        }
    }
}

static CTR: AtomicU64 = AtomicU64::new(0);

impl FromOp for Lang {
    type Error = String;
    fn from_op(op: &str, children: Vec<Id>) -> Result<Self, Self::Error> {
        match op {
            "+" => Ok(Lang::Add([children[0], children[1]])),
            "-" => Ok(Lang::Sub([children[0], children[1]])),
            "*" => Ok(Lang::Mul([children[0], children[1]])),
            "&" => Ok(Lang::BAnd([children[0], children[1]])),
            "|" => Ok(Lang::BOr([children[0], children[1]])),
            "~" => Ok(Lang::BNot(children[0])),
            "&&" => Ok(Lang::And([children[0], children[1]])),
            "||" => Ok(Lang::Or([children[0], children[1]])),
            "!" => Ok(Lang::Not(children[0])),
            "==" => Ok(Lang::ICmp(Cond::Eq, [children[0], children[1]])),
            "!=" => Ok(Lang::ICmp(Cond::Neq, [children[0], children[1]])),
            "<" => Ok(Lang::ICmp(Cond::Lt, [children[0], children[1]])),
            ">" => Ok(Lang::ICmp(Cond::Gt, [children[0], children[1]])),
            "<=" => Ok(Lang::ICmp(Cond::Leq, [children[0], children[1]])),
            ">=" => Ok(Lang::ICmp(Cond::Geq, [children[0], children[1]])),
            "phi" => Ok(Lang::Phi([children[0], children[1], children[2]])),
            "eval" => Ok(Lang::Eval([children[0], children[1]])),
            "pass" => Ok(Lang::Pass(children[0])),
            "alloca" => {
                let val = CTR.load(std::sync::atomic::Ordering::Relaxed);
                CTR.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                Ok(Lang::Alloca(val))
            }
            "ptr" => Ok(Lang::Ptr(children[0])),
            "load" => Ok(Lang::Load([children[0], children[1]])),
            "store" => Ok(Lang::Store([children[0], children[1], children[2]])),
            "true" => Ok(Lang::I1(true)),
            "false" => Ok(Lang::I1(false)),
            other => {
                let split: Vec<&str> = other.split_terminator('_').collect();
                let [val, ty] = split.as_slice() else {
                    Err(format!("Ill-formated value type: {}", op))?
                };
                match *ty {
                    "i64" => Ok(Lang::I64(val.parse().unwrap())),
                    "v" => Ok(Lang::Var(val.parse().unwrap())),
                    _ => Err(format!("Unknown operator: {}", op))?,
                }
            }
        }
    }
}

impl Display for Lang {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Lang::*;
        match self {
            Add(_) => write!(f, "+"),
            Sub(_) => write!(f, "-"),
            Mul(_) => write!(f, "*"),
            BAnd(_) => write!(f, "&"),
            BOr(_) => write!(f, "|"),
            BNot(_) => write!(f, "~"),
            And(_) => write!(f, "&&"),
            Or(_) => write!(f, "||"),
            Not(_) => write!(f, "!"),
            ICmp(cond, _) => match cond {
                Cond::Eq => write!(f, "=="),
                Cond::Neq => write!(f, "!="),
                Cond::Lt => write!(f, "<"),
                Cond::Gt => write!(f, ">"),
                Cond::Leq => write!(f, "<="),
                Cond::Geq => write!(f, ">="),
            },
            Phi(_) => write!(f, "phi"),
            Eval(_) => write!(f, "eval"),
            Pass(_) => write!(f, "pass"),
            Alloca(_) => write!(f, "alloca"),
            Ptr(_) => write!(f, "ptr"),
            Load(_) => write!(f, "load"),
            Store(_) => write!(f, "store"),
            I1(b) => write!(f, "{}", b),
            I64(i) => write!(f, "{}_i64", i),
            Var(s) => write!(f, "{}_v", s),
        }
    }
}
