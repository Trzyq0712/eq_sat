use egg::{Analysis, EGraph, merge_option, Id, DidMerge};
use super::Lang;

#[derive(Clone, Default)]
pub struct ConstFold;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Constant {
    Bool(bool),
    Num(i32),
}

impl Constant {
    fn get_bool(self) -> Option<bool> {
        use Constant::*;
        match self {
            Bool(b) => Some(b),
            Num(_) => None,
        }
    }
    fn get_num(self) -> Option<i32> {
        use Constant::*;
        match self {
            Num(n) => Some(n),
            Bool(_) => None,
        }
    }
}

impl Analysis<Lang> for ConstFold {
    type Data = Option<Constant>;

    fn merge(&mut self, to: &mut Self::Data, from: Self::Data) -> DidMerge {
        merge_option(to, from, |a, b| {
            assert_eq!(*a, b, "Merged non-equal constants");
            DidMerge(false, false)
        })
    }

    fn make(egraph: &EGraph<Lang, Self>, enode: &Lang) -> Self::Data {
        let x = |i: &Id| egraph[*i].data;
        let num = |n: i32| Some(Constant::Num(n));
        let boolean = |b: bool| Some(Constant::Bool(b));
        use Lang::*;
        match enode {
            Num(a) => num(*a),
            Neg(a) => num(-x(a)?.get_num()?),
            Add([a, b]) => num(x(a)?.get_num()? + x(b)?.get_num()?),
            Sub([a, b]) => num(x(a)?.get_num()? - x(b)?.get_num()?),
            Mul([a, b]) => num(x(a)?.get_num()? * x(b)?.get_num()?),

            Lt([l, r]) => boolean(x(l)?.get_num()? < x(r)?.get_num()?),
            Eq([l, r]) => boolean(x(l)?.get_num()? == x(r)?.get_num()?),
            Bool(b) => boolean(*b),
            Not(b) => boolean(x(b)?.get_bool()?),
            And([l, r]) => boolean(x(l)?.get_bool()? && x(r)?.get_bool()?),
            Or([l, r]) => boolean(x(l)?.get_bool()? || x(r)?.get_bool()?),
            _ => None,
        }
    }

    fn modify(egraph: &mut EGraph<Lang, Self>, id: Id) {
        if let Some(d) = egraph[id].data {
            let to_add = match d {
                Constant::Num(n) => Lang::Num(n),
                Constant::Bool(b) => Lang::Bool(b),
            };
            let added = egraph.add(to_add);
            egraph.union(id, added);
        }
    }

}
