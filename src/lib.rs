pub mod conv;
pub mod interp;
pub mod lang;
pub mod rules;

pub type EGraph = egg::EGraph<lang::Lang, ()>;
pub type Lang = lang::Lang;
