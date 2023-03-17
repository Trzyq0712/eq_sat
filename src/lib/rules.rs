use super::Lang;
use egg::*;

pub fn rw_rules() -> Vec<Rewrite<Lang, ()>> {
    arithmetic_rules()
}

fn arithmetic_rules() -> Vec<Rewrite<Lang, ()>> {
    let mut unidirectional = vec![
        rewrite!("commutative addition"; "(+ ?a ?b)" => "(+ ?b ?a)"),
        rewrite!("commutative multiplication"; "(* ?a ?b)" => "(* ?b ?a)"),
        // Temporary rules to be replaced with constant folding
        rewrite!("one multiplication"; "(* 1 ?a)" => "?a"),
        rewrite!("zero multiplication"; "(* 1 ?a)" => "?a"),
        rewrite!("zero addition"; "(+ 0 ?a)" => "?a"),
        rewrite!("three times five"; "(* 3 5)" => "15"),
        //
    ];
    let biderectional = vec![
        rewrite!("multiplication distribution over addition";
                 "(* (+ ?a ?b) ?m)" <=> "(+ (* ?a ?m) (* ?b ?m))"),
        rewrite!("loop multiplication"; "(* (theta ?init ?next) ?m)" <=> "(theta (* ?init ?m) (* ?next ?m))"),
        rewrite!("if multiplication"; "(* (phi ?cond ?then ?else) ?m)" <=> "(phi ?cond (* ?then ?m) (* ?else ?m))"),
    ].concat();
    unidirectional.extend(biderectional.into_iter());
    unidirectional
}
