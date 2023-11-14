// use lang::{Lang, analysis::ConstFold, rules::rw_rules};
// use egg::{EGraph, Runner};
//
// fn example_expression() {
//     // i := 0;
//     // while (...) {
//     //   use(i * 5);
//     //   i := i + 1;
//     //   if (...) {
//     //     i := i + 3;
//     //   }
//     // }
//
//     let mut graph = EGraph::default();
//
//     use Lang::*;
//     let zero = graph.add(Num(0));
//     let one = graph.add(Num(1));
//     let three = graph.add(Num(3));
//     let five = graph.add(Num(5));
//     let cond = graph.add(Symbol("c".into()));
//     let temp = graph.add(Temp(0));
//     let add1 = graph.add(Add([one, temp]));
//     let add2 = graph.add(Add([three, add1]));
//     let if_st = graph.add(If([cond, add2, add1]));
//     let loop_st = graph.add(Seq([zero, if_st]));
//     let times = graph.add(Mul([loop_st, five]));
//
//     graph.union(temp, loop_st);
//     graph.rebuild();
//
//     graph.dot().to_pdf("examples/ross-prio.pdf").unwrap();
//
//     let runner = Runner::default()
//         .with_explanations_enabled()
//         .with_egraph(graph)
//         .run(&rw_rules());
//
//     println!("Saving dot");
//     runner.egraph.dot().to_pdf("examples/ross.pdf").unwrap();
// }
//
// fn simple_constant_fold() {
//     let expr = "(+ 5 (phi (and true false) (* 1 2) (* 3 2)))".parse().unwrap();
//
//     let mut graph = EGraph::default();
//     graph.add_expr(&expr);
//
//     graph.dot().to_pdf("examples/constfold-prio.pdf").unwrap();
//
//     let runner = Runner::default().with_explanations_enabled().with_egraph(graph).run(&rw_rules());
//
//     runner.egraph.dot().to_pdf("examples/constfold.pdf").unwrap()
// }

fn main() {
    // example_expression();
    // simple_constant_fold();
    println!("Hello, world!");
}
