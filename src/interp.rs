use crate::lang::{Cond, Lang};

/// Value that can be a result of evaluating an expression
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    I64(i64),
    I1(bool),
    Ptr(usize),
    Sigma(usize),
}

/// Map from variable names to values
#[derive(Debug, Clone, Default)]
pub struct Env(std::collections::HashMap<egg::Symbol, Value>);

impl Env {
    fn get(&self, var: egg::Symbol) -> Option<Value> {
        self.0.get(&var).copied()
    }

    pub fn set(&mut self, var: egg::Symbol, val: Value) {
        self.0.insert(var, val);
    }
}

/// Map from (stack_slot, sigma) -> Value
#[derive(Debug, Clone, Default)]
pub struct Store(std::collections::HashMap<(usize, usize), Option<Value>>);

impl Store {
    fn get(&self, stack_slot: usize, sigma: usize) -> Option<Value> {
        self.0.get(&(stack_slot, sigma)).copied().flatten()
    }

    fn set(&mut self, stack_slot: usize, sigma: usize, val: Option<Value>) {
        self.0.insert((stack_slot, sigma), val);
    }
}

pub struct Expr<'a> {
    expr: &'a [Lang],
    root: usize,
}

impl<'a> Expr<'a> {
    pub fn new(expr: &'a egg::RecExpr<Lang>) -> Self {
        Self {
            expr: expr.as_ref(),
            root: expr.as_ref().len() - 1,
        }
    }

    pub fn with_root(expr: &'a egg::RecExpr<Lang>, root: egg::Id) -> Self {
        Self {
            expr: expr.as_ref(),
            root: usize::from(root),
        }
    }

    fn reroot(&self, root: usize) -> Self {
        Self {
            expr: self.expr,
            root,
        }
    }

    fn root(&self) -> &Lang {
        &self.expr[self.root]
    }

    pub fn interp(&self, env: &Env, st: &mut Store) -> Result<Value, String> {
        match *self.root() {
            Lang::I1(b) => Ok(Value::I1(b)),
            Lang::I64(i) => Ok(Value::I64(i)),
            Lang::Var(v) => env
                .get(v)
                .ok_or_else(|| format!("unbound variable `{}`", v)),
            Lang::Add([l, r]) => self.interp_binop(l, r, env, st, i64_binop(|l, r| l + r)),
            Lang::Sub([l, r]) => self.interp_binop(l, r, env, st, i64_binop(|l, r| l - r)),
            Lang::Mul([l, r]) => self.interp_binop(l, r, env, st, i64_binop(|l, r| l * r)),
            Lang::ICmp(cond, [l, r]) => self.interp_cond(cond, l, r, env, st),
            Lang::And([l, r]) => self.interp_binop(l, r, env, st, i1_binop(|l, r| l && r)),
            Lang::Or([l, r]) => self.interp_binop(l, r, env, st, i1_binop(|l, r| l || r)),
            Lang::Not(op) => {
                let op = self.reroot(usize::from(op)).interp(env, st)?;
                match op {
                    Value::I1(b) => Ok(Value::I1(!b)),
                    _ => Err(format!("cannot apply `not` to {:?}", op)),
                }
            }
            Lang::Phi([cnd, t, f]) => {
                let cnd = self.reroot(usize::from(cnd)).interp(env, st)?;
                match cnd {
                    Value::I1(true) => self.reroot(usize::from(t)).interp(env, st),
                    Value::I1(false) => self.reroot(usize::from(f)).interp(env, st),
                    _ => Err(format!("cannot interp {:?}", cnd)),
                }
            }
            Lang::Alloca(_) => Ok(Value::Sigma(self.root)),
            Lang::Ptr(sig) => {
                let sig = self.reroot(usize::from(sig)).interp(env, st)?;
                let Value::Sigma(sig) = sig else {
                    return Err("Expected a sigma".to_string());
                };

                let ptr = self.root;
                st.set(ptr, sig, None);
                Ok(Value::Ptr(ptr))
            }
            Lang::Store([val, sig, ptr]) => {
                let val = self.reroot(usize::from(val)).interp(env, st)?;
                let sig = self.reroot(usize::from(sig)).interp(env, st)?;
                let ptr = self.reroot(usize::from(ptr)).interp(env, st)?;
                let Value::Sigma(_sig) = sig else {
                    return Err("Expected a sigma".to_string());
                };
                let Value::Ptr(ptr) = ptr else {
                    return Err("Expected a pointer".to_string());
                };
                st.set(ptr, self.root, Some(val));
                Ok(Value::Sigma(self.root))
            }
            Lang::Load([sig, ptr]) => {
                let sig = self.reroot(usize::from(sig)).interp(env, st)?;
                let ptr = self.reroot(usize::from(ptr)).interp(env, st)?;
                let Value::Sigma(sig) = sig else {
                    return Err("Expected a sigma".to_string());
                };
                let Value::Ptr(ptr) = ptr else {
                    return Err("Expected a pointer".to_string());
                };
                st.get(ptr, sig)
                    .ok_or_else(|| "uninitialized value".to_string())
            }
            _ => Err(format!("cannot interp {:?}", self.root())),
        }
    }

    fn interp_binop(
        &self,
        l: egg::Id,
        r: egg::Id,
        env: &Env,
        st: &mut Store,
        op: impl FnOnce(Value, Value) -> Result<Value, String>,
    ) -> Result<Value, String> {
        let l = self.reroot(usize::from(l)).interp(env, st)?;
        let r = self.reroot(usize::from(r)).interp(env, st)?;
        op(l, r)
    }

    fn interp_cond(
        &self,
        cond: Cond,
        l: egg::Id,
        r: egg::Id,
        env: &Env,
        st: &mut Store,
    ) -> Result<Value, String> {
        let l = self.reroot(usize::from(l)).interp(env, st)?;
        let r = self.reroot(usize::from(r)).interp(env, st)?;
        match (l, r) {
            (Value::I64(l), Value::I64(r)) => eval_cond_i64(cond, l, r),
            (Value::I1(l), Value::I1(r)) => eval_cond_i1(cond, l, r),
            _ => Err(format!("cannot compare {:?} and {:?}", l, r)),
        }
    }
}

fn eval_cond_i64(cond: Cond, l: i64, r: i64) -> Result<Value, String> {
    Ok(Value::I1(match cond {
        Cond::Eq => l == r,
        Cond::Neq => l != r,
        Cond::Lt => l < r,
        Cond::Leq => l <= r,
        Cond::Gt => l > r,
        Cond::Geq => l >= r,
    }))
}

fn eval_cond_i1(cond: Cond, l: bool, r: bool) -> Result<Value, String> {
    match cond {
        Cond::Eq => Ok(l == r),
        Cond::Neq => Ok(l != r),
        _ => Err(format!(
            "cannot compare {:?} and {:?} using {:?}",
            l, r, cond
        )),
    }
    .map(Value::I1)
}

fn i64_binop(
    op: impl FnOnce(i64, i64) -> i64,
) -> impl FnOnce(Value, Value) -> Result<Value, String> {
    move |l, r| match (l, r) {
        (Value::I64(l), Value::I64(r)) => Ok(Value::I64(op(l, r))),
        _ => Err(format!("cannot apply op to {:?} and {:?}", l, r)),
    }
}

fn i1_binop(
    op: impl FnOnce(bool, bool) -> bool,
) -> impl FnOnce(Value, Value) -> Result<Value, String> {
    move |l, r| match (l, r) {
        (Value::I1(l), Value::I1(r)) => Ok(Value::I1(op(l, r))),
        _ => Err(format!("cannot apply op to {:?} and {:?}", l, r)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn interp_empty(expr: &Expr) -> Result<Value, String> {
        expr.interp(&Env::default(), &mut Store::default())
    }

    #[test]
    fn value_i64() {
        let mut expr = egg::RecExpr::default();
        expr.add(Lang::I64(1));
        let expr = Expr::new(&expr);
        assert_eq!(interp_empty(&expr), Ok(Value::I64(1)));
    }

    #[test]
    fn value_i1() {
        let mut expr = egg::RecExpr::default();
        expr.add(Lang::I1(true));
        let expr = Expr::new(&expr);
        assert_eq!(interp_empty(&expr), Ok(Value::I1(true)));
    }

    fn two_and_three() -> (egg::RecExpr<Lang>, egg::Id, egg::Id) {
        let mut expr = egg::RecExpr::default();
        let n1 = expr.add(Lang::I64(2));
        let n2 = expr.add(Lang::I64(3));
        (expr, n1, n2)
    }

    #[test]
    fn add() {
        let (mut expr, n1, n2) = two_and_three();
        expr.add(Lang::Add([n1, n2]));
        let expr = Expr::new(&expr);
        assert_eq!(interp_empty(&expr), Ok(Value::I64(5)));
    }

    #[test]
    fn sub() {
        let (mut expr, n1, n2) = two_and_three();
        expr.add(Lang::Sub([n1, n2]));
        let expr = Expr::new(&expr);
        assert_eq!(interp_empty(&expr), Ok(Value::I64(-1)));
    }

    #[test]
    fn compare_eq() {
        let (mut expr, n1, n2) = two_and_three();
        expr.add(Lang::ICmp(Cond::Eq, [n1, n2]));
        let expr = Expr::new(&expr);
        assert_eq!(interp_empty(&expr), Ok(Value::I1(false)));
    }

    #[test]
    fn compare_neq() {
        let (mut expr, n1, n2) = two_and_three();
        let res_tr = expr.add(Lang::ICmp(Cond::Neq, [n1, n2]));
        let res_fa = expr.add(Lang::ICmp(Cond::Neq, [n1, n1]));
        let expr_tr = Expr::with_root(&expr, res_tr.into());
        let expr_fa = Expr::with_root(&expr, res_fa.into());
        assert_eq!(interp_empty(&expr_tr), Ok(Value::I1(true)));
        assert_eq!(interp_empty(&expr_fa), Ok(Value::I1(false)));
    }

    #[test]
    fn compare_geq() {
        let (mut expr, n1, n2) = two_and_three();
        expr.add(Lang::ICmp(Cond::Geq, [n1, n2]));
        let expr = Expr::new(&expr);
        assert_eq!(interp_empty(&expr), Ok(Value::I1(false)));
    }

    #[test]
    fn compare_gt() {
        let (mut expr, n1, n2) = two_and_three();
        expr.add(Lang::ICmp(Cond::Gt, [n1, n2]));
        let expr = Expr::new(&expr);
        assert_eq!(interp_empty(&expr), Ok(Value::I1(false)));
    }

    #[test]
    fn conditional() {
        let mut expr = egg::RecExpr::default();
        let cond = expr.add(Lang::I1(false));
        let then_b = expr.add(Lang::I64(4));
        let else_b = expr.add(Lang::I64(5));
        let _cond_res = expr.add(Lang::Phi([cond, then_b, else_b]));
        let expr = Expr::new(&expr);
        assert_eq!(interp_empty(&expr), Ok(Value::I64(5)));
    }

    #[test]
    fn env_variables() {
        let s1 = egg::Symbol::new("x");
        let s2 = egg::Symbol::new("y");

        let env = Env([(s1, Value::I64(3)), (s2, Value::I64(1))].into());

        let mut expr = egg::RecExpr::default();
        let v1 = expr.add(Lang::Var(s1));
        let v2 = expr.add(Lang::Var(s2));
        let _sum = expr.add(Lang::Add([v1, v2]));
        let expr = Expr::new(&expr);

        assert_eq!(expr.interp(&env, &mut Store::default()), Ok(Value::I64(4)));
    }
}
