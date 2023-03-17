use egg::*;
mod rules;

define_language! {
    pub enum Lang {
        "+" = Add([Id; 2]),
        "-" = Sub([Id; 2]),
        "*" = Mul([Id; 2]),
        "/" = Div([Id; 2]),
        "-" = Neg(Id),
        "<" = Lt([Id; 2]),
        "==" = Eq([Id; 2]),
        "theta" = Seq([Id; 2]),
        "eval" = Eval([Id; 2]), // sequence, nth of
        "pass" = Pass(Id), // returns index of first true in sequence
        "phi" = If([Id; 3]), // cond, true, else
        Bool(bool),
        Num(i32),
        Temp(i32),
        Symbol(Symbol),
    }
}

#[cfg(test)]
mod tests {
    use egg::{EGraph, RecExpr, Runner};

    use crate::{rules, Lang};

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

    fn example_expression(graph: &mut EGraph<Lang, ()>) -> egg::Id {
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
        let tr = graph.add(Bool(true));
        let temp = graph.add(Temp(0));
        let add1 = graph.add(Add([one, temp]));
        let add2 = graph.add(Add([three, add1]));
        let if_st = graph.add(If([tr, add2, add1]));
        let loop_st = graph.add(Seq([zero, if_st]));
        let times = graph.add(Mul([loop_st, five]));

        graph.union(temp, loop_st);
        graph.rebuild();
        times
    }

    fn example_expression_simplified(graph: &mut EGraph<Lang, ()>) -> egg::Id {
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
        let tr = graph.add(Bool(true));
        let temp = graph.add(Temp(1));
        let add1 = graph.add(Add([five, temp]));
        let add2 = graph.add(Add([fifteen, add1]));
        let if_st = graph.add(If([tr, add2, add1]));
        let loop_st = graph.add(Seq([zero, if_st]));

        graph.union(temp, loop_st);
        graph.rebuild();
        loop_st
    }

    #[test]
    fn ross_tate_example() {
        let mut graph = EGraph::default();

        let id_example = example_expression(&mut graph);
        let id_example_simplified = example_expression(&mut graph);

        let runner = Runner::default()
            .with_explanations_enabled()
            .with_egraph(graph)
            .run(&rules::rw_rules());
        assert_eq!(
            runner.egraph.find(id_example),
            runner.egraph.find(id_example_simplified)
        );
    }
}
