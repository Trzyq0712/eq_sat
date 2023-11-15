use std::fmt::Display;
use std::fmt::Formatter;

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

#[derive(Clone, Debug, Hash, Ord, PartialOrd)]
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

    Alloca,
    Ptr(Id),
    Load([Id; 2]),
    Store([Id; 3]),

    I1(bool),
    I64(i64),
    Var(Symbol),
}

impl PartialEq for Lang {
    fn eq(&self, other: &Self) -> bool {
        use Lang::*;
        match (self, other) {
            (Add(a), Add(b)) => a == b,
            (Sub(a), Sub(b)) => a == b,
            (Mul(a), Mul(b)) => a == b,
            (BAnd(a), BAnd(b)) => a == b,
            (BOr(a), BOr(b)) => a == b,
            (BNot(a), BNot(b)) => a == b,
            (And(a), And(b)) => a == b,
            (Or(a), Or(b)) => a == b,
            (Not(a), Not(b)) => a == b,
            (ICmp(a, b), ICmp(c, d)) => a == c && b == d,
            (Phi(a), Phi(b)) => a == b,
            (Alloca, Alloca) => false,
            (Ptr(a), Ptr(b)) => a == b,
            (Load(a), Load(b)) => a == b,
            (Store(a), Store(b)) => a == b,
            (I1(a), I1(b)) => a == b,
            (I64(a), I64(b)) => a == b,
            (Var(a), Var(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Lang {}

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
            (Alloca, Alloca) => false,
            (Ptr(_), Ptr(_)) => true,
            (Load(_), Load(_)) => true,
            (Store(_), Store(_)) => true,
            (I1(a), I1(b)) => a == b,
            (I64(a), I64(b)) => a == b,
            (Var(a), Var(b)) => a == b,
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

            Lang::Alloca => &[],
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

            Lang::Alloca => &mut [],
            Lang::Ptr(op) => std::slice::from_mut(op),
            Lang::Load(ops) => ops,
            Lang::Store(ops) => ops,

            Lang::I1(_) => &mut [],
            Lang::I64(_) => &mut [],
            Lang::Var(_) => &mut [],
        }
    }
}

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
            "alloca" => Ok(Lang::Alloca),
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
            Alloca => write!(f, "alloca"),
            Ptr(_) => write!(f, "ptr"),
            Load(_) => write!(f, "load"),
            Store(_) => write!(f, "store"),
            I1(b) => write!(f, "{}", b),
            I64(i) => write!(f, "{}_i64", i),
            Var(s) => write!(f, "{}_v", s),
        }
    }
}

// define_language! {
// pub enum Lang {
//     // Binops
//     "+" = Add([Id; 2]),
//     "-" = Sub([Id; 2]),
//     "*" = Mul([Id; 2]),
//     "/" = Div([Id; 2]),
//     "&" = BAnd([Id; 2]), // bitwise and --> lhs: i64, rhs: i64 -> i64
//     "|" = BOr([Id; 2]),  // bitwise or --> lhs: i64, rhs: i64 -> i64
//     "~" = BNot(Id),      // bitwise not --> i64 -> i64
//
//     // Unops
//     "-" = Neg(Id),
//
//     // Comparisons
//     // FIXME: should be able to use all predicates
//     "cmp" = ICmp([Id; 2]), // comparison --> op: (eq, neq, l, g, leq, geq), lhs: i64, rhs: i64 -> i1
//     "&&" = And([Id; 2]),  // logical and --> lhs: i1, rhs: i1 -> i1
//     "||" = Or([Id; 2]),   // logical or --> lhs: i1, rhs: i1 -> i1
//     "!" = Not(Id),        // logical not --> i1 -> i1
//
//     "phi" = Phi([Id; 3]), // cond, true, else
//
//     Alloca(usize),
//     "ptr" = Ptr(Id),          // i64*
//     "load" = Load([Id; 2]),   // witness: sigma, ptr: i64* -> i64
//     "store" = Store([Id; 3]), // witness: sigma, ptr: i64*, val: i64 -> sigma
//
//     Bool(bool), // i1
//     Num(i64),
//     Symbol(Symbol),
//     Temp(usize),
//
//     "pass" = Pass(Id), // returns index of first true in sequence
//     "seq" = Seq([Id; 2]),
//     "eval" = Eval([Id; 2]), // sequence, nth of
// }
// }

// #[cfg(test)]
// mod tests {
//     use egg::{EGraph, RecExpr, Runner};
//
//     use crate::{analysis, rules, Lang};
//
//     #[test]
//     fn lang() {
//         use Lang::*;
//         let mut expr = RecExpr::default();
//         let one = expr.add(Num(1));
//         let two = expr.add(Num(2));
//         expr.add(Add([one, two]));
//         let s = "(+ 1 2)";
//         let parsed = s.parse().unwrap();
//         assert_eq!(expr, parsed);
//     }
//
//     fn example_expression(graph: &mut EGraph<Lang, analysis::ConstFold>) -> egg::Id {
//         // i := 0;
//         // while (...) {
//         //   use(i * 5);
//         //   i := i + 1;
//         //   if (...) {
//         //     i := i + 3;
//         //   }
//         // }
//
//         use Lang::*;
//         let zero = graph.add(Num(0));
//         let one = graph.add(Num(1));
//         let three = graph.add(Num(3));
//         let five = graph.add(Num(5));
//         let cond = graph.add(Symbol("c".into()));
//         let temp = graph.add(Temp(0));
//         let add1 = graph.add(Add([one, temp]));
//         let add2 = graph.add(Add([three, add1]));
//         let if_st = graph.add(If([cond, add2, add1]));
//         let loop_st = graph.add(Seq([zero, if_st]));
//         let times = graph.add(Mul([loop_st, five]));
//
//         graph.union(temp, loop_st);
//         graph.rebuild();
//         println!("normal root id: {}", times);
//         times
//     }
//
//     fn example_expression_simplified(graph: &mut EGraph<Lang, analysis::ConstFold>) -> egg::Id {
//         // i := 0;
//         // while (...) {
//         //   use(i);
//         //   i := i + 5;
//         //   if (...) {
//         //     i := i + 15;
//         //   }
//         // }
//
//         use Lang::*;
//
//         let zero = graph.add(Num(0));
//         let five = graph.add(Num(5));
//         let fifteen = graph.add(Num(15));
//         let cond = graph.add(Symbol("c".into()));
//         let temp = graph.add(Temp(1));
//         let add1 = graph.add(Add([five, temp]));
//         let add2 = graph.add(Add([fifteen, add1]));
//         let if_st = graph.add(If([cond, add2, add1]));
//         let loop_st = graph.add(Seq([zero, if_st]));
//
//         graph.union(temp, loop_st);
//         graph.rebuild();
//         println!("simplified root id: {}", loop_st);
//         loop_st
//     }
//
//     #[test]
//     fn ross_tate_example() {
//         let mut graph = EGraph::default();
//
//         let id_example = example_expression(&mut graph);
//         let id_example_simplified = example_expression_simplified(&mut graph);
//
//         let runner = Runner::default()
//             .with_explanations_enabled()
//             .with_egraph(graph)
//             .run(&rules::rw_rules());
//
//         println!("{:?}", &runner.egraph);
//         // println!("{:?}", runner.egraph.clone().with_explanations_enabled().id_to_expr(id_example));
//         // println!("{:?}", runner.egraph.clone().with_explanations_enabled().id_to_expr(id_example_simplified));
//         runner.egraph.dot().to_pdf("test.pdf");
//         assert_eq!(
//             runner.egraph.find(id_example),
//             runner.egraph.find(id_example_simplified)
//         );
//     }
//
//     #[test]
//     fn test_simple_equality() {
//         let mut graph = EGraph::default();
//
//         let id1 = {
//             let expr1 = "(+ 4 (phi c 2 3))".parse().unwrap();
//             graph.add_expr(&expr1)
//         };
//
//         let id2 = {
//             let expr2 = "(* 1 (+ (phi (not c) 3 2) 4))".parse().unwrap();
//             graph.add_expr(&expr2)
//         };
//
//         let runner = Runner::default()
//             .with_explanations_enabled()
//             .with_egraph(graph)
//             .run(&rules::rw_rules());
//
//         assert_eq!(runner.egraph.find(id1), runner.egraph.find(id2));
//     }
//
//     #[test]
//     fn test_identity() {
//         let mut graph = EGraph::default();
//
//         let alloc = graph.add(Lang::Alloca(0));
//         let sigma = graph.add(Lang::Sigma(alloc));
//         let ptr = graph.add(Lang::Ptr(alloc));
//         let x = graph.add(Lang::Symbol("x".into()));
//         let store = graph.add(Lang::Store([sigma, ptr, x]));
//         let load = graph.add(Lang::Load([store, ptr]));
//
//         graph.dot().to_pdf("identity-orig.pdf");
//
//         let runner = Runner::default()
//             .with_explanations_enabled()
//             .with_egraph(graph)
//             .run(&rules::rw_rules());
//
//         runner.egraph.dot().to_pdf("identity-sat.pdf");
//     }
// }
