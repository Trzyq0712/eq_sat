use super::Lang;
use crate::analysis::ConstFold;
use egg::*;

pub fn rw_rules() -> Vec<Rewrite<Lang, ConstFold>> {
    arithmetic_rules()
}

fn arithmetic_rules() -> Vec<Rewrite<Lang, ConstFold>>  {
    let mut unidirectional = vec![
        rewrite!("commutative addition"; "(+ ?a ?b)" => "(+ ?b ?a)"),
        rewrite!("commutative multiplication"; "(* ?a ?b)" => "(* ?b ?a)"),
        rewrite!("double minus"; "(- (- ?a))" => "?a"),
        rewrite!("double not"; "(not (not ?a))" => "?a"),
        rewrite!("comm and"; "(and ?a ?b)" => "(and ?b ?a)"),
        rewrite!("comm or"; "(or ?a ?b)" => "(or ?b ?a)"),
        rewrite!("not if"; "(phi ?c ?t ?e)" => "(phi (not ?c) ?e ?t)"),
        rewrite!("one multiplication"; "(* 1 ?a)" => "?a"),
        rewrite!("zero multiplication"; "(* 1 ?a)" => "?a"),
        rewrite!("zero addition"; "(+ 0 ?a)" => "?a"),
    ];

    let biderectional = vec![
        rewrite!("multiplication distribution over addition";
                 "(* (+ ?a ?b) ?m)" <=> "(+ (* ?a ?m) (* ?b ?m))"),
        rewrite!("loop multiplication"; "(* (theta ?init ?next) ?m)" <=> "(theta (* ?init ?m) (* ?next ?m))"),
        rewrite!("if multiplication"; "(* (phi ?c ?t ?e) ?m)" <=> "(phi ?c (* ?t ?m) (* ?e ?m))"),
    ].concat();

    unidirectional.extend(biderectional.into_iter());
    unidirectional
}
