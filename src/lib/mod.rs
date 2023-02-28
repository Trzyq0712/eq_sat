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
        Symbol(Symbol),
    }
}

#[cfg(test)]
mod tests {}
