pub mod conv;
pub mod lang;
mod rules;

type EGraph = egg::EGraph<lang::Lang, ()>;
type Lang = lang::Lang;
