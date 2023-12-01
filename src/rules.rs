use crate::lang::Lang;
use egg::{rewrite, Rewrite};

pub fn rw_rules() -> Vec<Rewrite<Lang, ()>> {
    let mut rules = vec![];
    rules.extend(allocation_rules());
    rules.extend(arithmetic_rules());
    rules.extend(phi_rules());
    rules.extend(logic_rules());
    rules.extend(cond_rules());
    rules
}

fn arithmetic_rules() -> Vec<Rewrite<Lang, ()>> {
    let mut unidirectional = vec![
        //        rewrite!("commutative addition"; "(+ ?a ?b)" => "(+ ?b ?a)"),
        //        rewrite!("commutative multiplication"; "(* ?a ?b)" => "(* ?b ?a)"),
        //        rewrite!("double minus"; "(- (- ?a))" => "?a"),
        //        rewrite!("double not"; "(not (not ?a))" => "?a"),
        //        rewrite!("not if"; "(phi ?c ?t ?e)" => "(phi (not ?c) ?e ?t)"),
        rewrite!("one multiplication"; "(* 1_i64 ?a)" => "?a"),
        rewrite!("zero multiplication"; "(* 0_i64 ?a)" => "0_i64"),
        rewrite!("zero addition"; "(+ 0_i64 ?a)" => "?a"),
    ];

    let biderectional = vec![
        rewrite!("multiplication distribution over addition";
                 "(* (+ ?a ?b) ?m)" <=> "(+ (* ?a ?m) (* ?b ?m))"),
        // rewrite!("loop multiplication"; "(* (theta ?init ?next) ?m)" <=> "(theta (* ?init ?m) (* ?next ?m))"),
        // rewrite!("if multiplication"; "(* (phi ?c ?t ?e) ?m)" <=> "(phi ?c (* ?t ?m) (* ?e ?m))"),
    ]
    .concat();

    unidirectional.extend(biderectional.into_iter());
    unidirectional
}

fn allocation_rules() -> Vec<Rewrite<Lang, ()>> {
    vec![
        rewrite!("drop store"; "(load (store ?v ?s ?p) ?p)" => "?v"),
        rewrite!("lower load over phi"; "(load (phi ?c ?t ?e) ?p)" => "(phi ?c (load ?t ?p) (load ?e ?p))"),
    ]
}

fn phi_rules() -> Vec<Rewrite<Lang, ()>> {
    let mut uni = vec![
        rewrite!("phi if true"; "(phi true ?t ?e)" => "?t"),
        rewrite!("phi if false"; "(phi false ?t ?e)" => "?e"),
        rewrite!("phi if same"; "(phi ?c ?t ?t)" => "?t"),
        rewrite!("phi if neg"; "(phi ?c ?t ?e)" => "(phi (! ?c) ?e ?t)"),
    ];
    let bi = vec![
        rewrite!("phi and"; "(phi (&& ?c1 ?c2) ?t ?e)" <=> "(phi ?c1 (phi ?c2 ?t ?e) ?e)"),
        rewrite!("phi or"; "(phi (|| ?c1 ?c2) ?t ?e)" <=> "(phi ?c1 ?t (phi ?c2 ?t ?e))"),
        rewrite!("phi cond already true"; "(phi (&& ?c1 ?c2) (phi (&& ?c1 ?c3) ?t2 ?e2) ?e1)" 
        <=> "(phi (&& ?c1 ?c2) (phi ?c3 ?t2 ?e2) ?e1)"),
    ]
    .concat();
    uni.extend(bi.into_iter());
    uni
}

fn cond_rules() -> Vec<Rewrite<Lang, ()>> {
    let uni = vec![
        rewrite!("not eq"; "(! (== ?a ?b))" => "(!= ?a ?b)"),
        rewrite!("not neq"; "(! (!= ?a ?b))" => "(== ?a ?b)"),
        rewrite!("not lt"; "(! (< ?a ?b))" => "(>= ?a ?b)"),
        rewrite!("not gt"; "(! (> ?a ?b))" => "(<= ?a ?b)"),
        rewrite!("lt or gt"; "(|| (< ?a ?b) (> ?a ?b))" => "(!= ?a ?b)"),
        rewrite!("lte and gte"; "(&& (<= ?a ?b) (>= ?a ?b))" => "(== ?a ?b)"),
        rewrite!("lt and eq"; "(&& (< ?a ?b) (== ?a ?b))" => "(<= ?a ?b)"),
        rewrite!("gt and eq"; "(&& (> ?a ?b) (== ?a ?b))" => "(>= ?a ?b)"),
        rewrite!("lte and neq"; "(&& (<= ?a ?b) (!= ?a ?b))" => "(< ?a ?b)"),
        rewrite!("gte and neq"; "(&& (>= ?a ?b) (!= ?a ?b))" => "(> ?a ?b)"),
        rewrite!("comm eq"; "(== ?a ?b)" => "(== ?b ?a)"),
        rewrite!("comm neq"; "(!= ?a ?b)" => "(!= ?b ?a)"),
        rewrite!("comm lt"; "(< ?a ?b)" => "(> ?b ?a)"),
        rewrite!("comm gt"; "(> ?a ?b)" => "(< ?b ?a)"),
        rewrite!("comm lte"; "(<= ?a ?b)" => "(>= ?b ?a)"),
        rewrite!("comm gte"; "(>= ?a ?b)" => "(<= ?b ?a)"),
        rewrite!("lt lte false"; "(&& (< ?a ?b) (<= ?b ?a))" => "false"),
        rewrite!("lt lt false"; "(&& (< ?a ?b) (< ?b ?a))" => "false"),
        rewrite!("gt gte false"; "(&& (> ?a ?b) (>= ?b ?a))" => "false"),
        rewrite!("gt gt false"; "(&& (> ?a ?b) (> ?b ?a))" => "false"),
        rewrite!("lt lte true"; "(|| (< ?a ?b) (<= ?b ?a))" => "true"),
        rewrite!("gt gte true"; "(|| (> ?a ?b) (>= ?b ?a))" => "true"),
    ];
    uni
}

fn logic_rules() -> Vec<Rewrite<Lang, ()>> {
    let mut uni = vec![
        rewrite!("not true"; "(! true)" => "false"),
        rewrite!("not false"; "(! false)" => "true"),
        rewrite!("double not"; "(! (! ?a))" => "?a"),
        rewrite!("comm and"; "(&& ?a ?b)" => "(&& ?b ?a)"),
        rewrite!("comm or"; "(|| ?a ?b)" => "(|| ?b ?a)"),
        rewrite!("assoc and"; "(&& ?a (&& ?b ?c))" => "(&& (&& ?a ?b) ?c)"),
        rewrite!("assoc or"; "(|| ?a (|| ?b ?c))" => "(|| (|| ?a ?b) ?c)"),
        rewrite!("ident and"; "(&& ?a true)" => "?a"),
        rewrite!("ident or"; "(|| ?a false)" => "?a"),
        rewrite!("zero and"; "(&& ?a false)" => "false"),
        rewrite!("zero or"; "(|| ?a true)" => "true"),
    ];
    let bi = vec![
        rewrite!("distributive and"; "(&& ?a (|| ?b ?c))" <=> "(|| (&& ?a ?b) (&& ?a ?c))"),
        rewrite!("distributive or"; "(|| ?a (&& ?b ?c))" <=> "(&& (|| ?a ?b) (|| ?a ?c))"),
    ]
    .concat();
    uni.extend(bi.into_iter());
    uni
}
