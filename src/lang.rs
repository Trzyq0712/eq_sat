use egg::{define_language, Id, Symbol};

define_language! {
pub enum Lang {
    // Binops
    "+" = Add([Id; 2]),
    "-" = Sub([Id; 2]),
    "*" = Mul([Id; 2]),
    "/" = Div([Id; 2]),
    "&" = BAnd([Id; 2]), // bitwise and --> lhs: i64, rhs: i64 -> i64
    "|" = BOr([Id; 2]),  // bitwise or --> lhs: i64, rhs: i64 -> i64
    "~" = BNot(Id),      // bitwise not --> i64 -> i64

    // Unops
    "-" = Neg(Id),

    // Comparisons
    // FIXME: should be able to use all predicates
    "cmp" = ICmp([Id; 2]), // comparison --> op: (eq, neq, l, g, leq, geq), lhs: i64, rhs: i64 -> i1
    "&&" = And([Id; 2]),  // logical and --> lhs: i1, rhs: i1 -> i1
    "||" = Or([Id; 2]),   // logical or --> lhs: i1, rhs: i1 -> i1
    "!" = Not(Id),        // logical not --> i1 -> i1

    "phi" = Phi([Id; 3]), // cond, true, else

    Alloca(usize),
    "ptr" = Ptr(Id),          // i64*
    "load" = Load([Id; 2]),   // witness: sigma, ptr: i64* -> i64
    "store" = Store([Id; 3]), // witness: sigma, ptr: i64*, val: i64 -> sigma

    Bool(bool), // i1
    Num(i64),
    Symbol(Symbol),
    Temp(usize),

    "pass" = Pass(Id), // returns index of first true in sequence
    "seq" = Seq([Id; 2]),
    "eval" = Eval([Id; 2]), // sequence, nth of
}
}

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
