use egg::{AstDepth, AstSize, EGraph, Extractor, Runner};
use lang::conv::to_epeg::parse_function;
use lang::rules::rw_rules;
use std::env;
use std::io::stdin;
use std::path::Path;

fn main() {
    // let file = "tests/llvm_parser/nested_ifs.bc";
    let file = "llvm_programs/complex_ifs/complex_ifs.bc";
    let path = Path::new(file);
    dbg!(&path);
    let module = llvm_ir::Module::from_bc_path(path).unwrap();
    // dbg!(&module.functions);

    let func = &module.functions[0];

    let (egraph, root) = parse_function(func);

    // let extractor = Extractor::new(&egraph, AstSize);
    // let (best_cost, best) = extractor.find_best(root);
    // println!("Extracted: \n{}", best.pretty(20));
    // egraph.dot().to_pdf("examples/if.pdf").unwrap();

    let runner = Runner::default()
        .with_node_limit(1000000)
        .with_time_limit(std::time::Duration::from_secs(15))
        .with_iter_limit(100)
        .with_egraph(egraph)
        .run(&rw_rules());
    dbg!(runner.stop_reason.unwrap());

    println!("Root: {:?}", &root);
    println!("Root: {:?}", &runner.egraph.find(root));
    // println!("Egraph: \n {:?}", &runner.egraph);
    let extractor = Extractor::new(&runner.egraph, AstDepth);
    let (best_cost, best) = extractor.find_best(root);

    println!("Extracted: \n{}", best.pretty(20));

    let mut egraph = EGraph::default();
    let root = egraph.add_expr(&best);
    egraph.dot().to_pdf("examples/if.pdf").unwrap();

    let runner = Runner::default()
        .with_node_limit(1000000)
        .with_time_limit(std::time::Duration::from_secs(15))
        .with_iter_limit(100)
        .with_egraph(egraph)
        .run(&rw_rules());
    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (best_cost, best) = extractor.find_best(root);
    println!("Extracted: \n{}", best.pretty(20));
    // runner.egraph.dot().to_pdf("examples/if.pdf").unwrap();
}
