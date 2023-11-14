use egg::*;
pub mod analysis;
mod llvm_conv;
pub mod rules;

define_language! {
    pub enum Lang {
        "+" = Add([Id; 2]),
        "-" = Sub([Id; 2]),
        "*" = Mul([Id; 2]),
        "/" = Div([Id; 2]),
        "-" = Neg(Id),
        "<" = Lt([Id; 2]),
        "==" = Eq([Id; 2]),
        "and" = And([Id; 2]),
        "or" = Or([Id; 2]),
        "not" = Not(Id),
        "theta" = Seq([Id; 2]),
        "eval" = Eval([Id; 2]), // sequence, nth of
        "pass" = Pass(Id), // returns index of first true in sequence
        "phi" = If([Id; 3]), // cond, true, else
        "sigma" = Sigma(Id), // sigma-token
        "store" = Store([Id; 3]), // sigma-token, pointer, value -> sigma-token
        "load" = Load([Id; 2]), // sigma-token, pointer -> value
        "ptr" = Ptr(Id),
        Bool(bool),
        Num(i32),
        Temp(usize),
        Alloca(usize),
        Symbol(Symbol),
    }
}

#[cfg(test)]
mod tests {
    use egg::{EGraph, RecExpr, Runner};

    use crate::{analysis, rules, Lang};

    #[test]
    fn lang() {
        use Lang::*;
        let mut expr = RecExpr::default();
        let one = expr.add(Num(1));
        let two = expr.add(Num(2));
        expr.add(Add([one, two]));
        let s = "(+ 1 2)";
        let parsed = s.parse().unwrap();
        assert_eq!(expr, parsed);
    }

    fn example_expression(graph: &mut EGraph<Lang, analysis::ConstFold>) -> egg::Id {
        // i := 0;
        // while (...) {
        //   use(i * 5);
        //   i := i + 1;
        //   if (...) {
        //     i := i + 3;
        //   }
        // }

        use Lang::*;
        let zero = graph.add(Num(0));
        let one = graph.add(Num(1));
        let three = graph.add(Num(3));
        let five = graph.add(Num(5));
        let cond = graph.add(Symbol("c".into()));
        let temp = graph.add(Temp(0));
        let add1 = graph.add(Add([one, temp]));
        let add2 = graph.add(Add([three, add1]));
        let if_st = graph.add(If([cond, add2, add1]));
        let loop_st = graph.add(Seq([zero, if_st]));
        let times = graph.add(Mul([loop_st, five]));

        graph.union(temp, loop_st);
        graph.rebuild();
        println!("normal root id: {}", times);
        times
    }

    fn example_expression_simplified(graph: &mut EGraph<Lang, analysis::ConstFold>) -> egg::Id {
        // i := 0;
        // while (...) {
        //   use(i);
        //   i := i + 5;
        //   if (...) {
        //     i := i + 15;
        //   }
        // }

        use Lang::*;

        let zero = graph.add(Num(0));
        let five = graph.add(Num(5));
        let fifteen = graph.add(Num(15));
        let cond = graph.add(Symbol("c".into()));
        let temp = graph.add(Temp(1));
        let add1 = graph.add(Add([five, temp]));
        let add2 = graph.add(Add([fifteen, add1]));
        let if_st = graph.add(If([cond, add2, add1]));
        let loop_st = graph.add(Seq([zero, if_st]));

        graph.union(temp, loop_st);
        graph.rebuild();
        println!("simplified root id: {}", loop_st);
        loop_st
    }

    #[test]
    fn ross_tate_example() {
        let mut graph = EGraph::default();

        let id_example = example_expression(&mut graph);
        let id_example_simplified = example_expression_simplified(&mut graph);

        let runner = Runner::default()
            .with_explanations_enabled()
            .with_egraph(graph)
            .run(&rules::rw_rules());

        println!("{:?}", &runner.egraph);
        // println!("{:?}", runner.egraph.clone().with_explanations_enabled().id_to_expr(id_example));
        // println!("{:?}", runner.egraph.clone().with_explanations_enabled().id_to_expr(id_example_simplified));
        runner.egraph.dot().to_pdf("test.pdf");
        assert_eq!(
            runner.egraph.find(id_example),
            runner.egraph.find(id_example_simplified)
        );
    }

    #[test]
    fn test_simple_equality() {
        let mut graph = EGraph::default();

        let id1 = {
            let expr1 = "(+ 4 (phi c 2 3))".parse().unwrap();
            graph.add_expr(&expr1)
        };

        let id2 = {
            let expr2 = "(* 1 (+ (phi (not c) 3 2) 4))".parse().unwrap();
            graph.add_expr(&expr2)
        };

        let runner = Runner::default()
            .with_explanations_enabled()
            .with_egraph(graph)
            .run(&rules::rw_rules());

        assert_eq!(runner.egraph.find(id1), runner.egraph.find(id2));
    }

    #[test]
    fn test_identity() {
        let mut graph = EGraph::default();

        let alloc = graph.add(Lang::Alloca(0));
        let sigma = graph.add(Lang::Sigma(alloc));
        let ptr = graph.add(Lang::Ptr(alloc));
        let x = graph.add(Lang::Symbol("x".into()));
        let store = graph.add(Lang::Store([sigma, ptr, x]));
        let load = graph.add(Lang::Load([store, ptr]));

        graph.dot().to_pdf("identity-orig.pdf");

        let runner = Runner::default()
            .with_explanations_enabled()
            .with_egraph(graph)
            .run(&rules::rw_rules());

        runner.egraph.dot().to_pdf("identity-sat.pdf");
    }
}
