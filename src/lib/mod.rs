use egg::*;

define_language! {
    enum Lang {
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
    use egg::{EGraph, RecExpr};

    use crate::Lang;

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

    #[test]
    fn ross_tate_example() {
        // i := 0;
        // while (...) {
        //   use(i * 5);
        //   i := i + 1;
        //   if (...) {
        //     i := i + 3;
        //   }
        // }

        use Lang::*;
        let mut expr = RecExpr::default();
        let zero = expr.add(Num(0));
        let one = expr.add(Num(1));
        let three = expr.add(Num(3));
        let five = expr.add(Num(5));
        let tr = expr.add(Bool(true));
        let temp = expr.add(Temp(0));
        let add1 = expr.add(Add([one, temp]));
        let add2 = expr.add(Add([three, add1]));
        let if_st = expr.add(If([tr, add2, add1]));
        let loop_st = expr.add(Seq([zero, if_st]));
        let times = expr.add(Mul([loop_st, five]));

        let mut graph: EGraph<Lang, ()> = EGraph::default().with_explanations_enabled();
        graph.add_expr(&expr);
        graph.union(temp, loop_st);
        graph.rebuild();
        let expr = graph.id_to_expr(times);
        dbg!(expr.is_dag());
        dbg!(graph.dump());
        panic!();
    }
}
