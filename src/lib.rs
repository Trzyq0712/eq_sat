pub mod conv;
pub mod cost_fn;
pub mod interp;
pub mod lang;
pub mod rules;

pub type EGraph = egg::EGraph<lang::Lang, ()>;
pub type Lang = lang::Lang;

pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: egg::RecExpr<Lang>,
    pub root: egg::Id,
}
