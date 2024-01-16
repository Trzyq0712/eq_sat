use crate::Lang;

use egg::{CostFunction, Language};

pub struct NoAlloc;

impl CostFunction<Lang> for NoAlloc {
    type Cost = f64;

    fn cost<C>(&mut self, enode: &Lang, mut costs: C) -> Self::Cost
    where
        C: FnMut(egg::Id) -> Self::Cost,
    {
        let (own_cost, multiplier) = match enode {
            // Simple arithmetic operations should be cheap
            Lang::Add(_) | Lang::Sub(_) => (1.0, 1.0),
            // Multiplications are more expensive
            Lang::Mul(_) => (4.0, 1.0),
            // Bitwise operations are cheap
            Lang::BAnd(_)
            | Lang::BOr(_)
            | Lang::BNot(_)
            | Lang::And(_)
            | Lang::Or(_)
            | Lang::Not(_) => (0.5, 1.0),
            // Comparisons are quite expensive
            Lang::ICmp(..) => (3.0, 1.0),
            // Memory operations are very expensive
            Lang::Load(_) | Lang::Store(_) => (10.0, 1.0),
            // Control flow is expensive, additional multiplier for inner nodes
            Lang::Phi(_) => (10.0, 5.0),

            // We don not want to allocate memory
            Lang::Alloca(_) => (1000.0, 0.0),
            // Constants are very cheap
            Lang::Ptr(_) | Lang::I1(_) | Lang::I64(_) => (0.01, 0.0),
            // "Variables" are cheap, but we want to avoid them if possible
            Lang::Var(_) => (0.1, 0.0),
            _ => todo!("Undefined cost for node {:?}", enode),
        };

        own_cost + multiplier * enode.fold(0.0, |acc, id| acc + costs(id))
    }
}
