use egg::{AstDepth, AstSize, Extractor, Runner};
use lang::conv::to_epeg::parse_function;
use lang::rules::rw_rules;
use lang::EGraph;
use std::env;
use std::io::stdin;
use std::path::Path;

fn main() {
    let file = env::args().nth(1).expect("No LLVM bytecode file provided");
    // let file = "llvm_programs/triple_if/triple_if.bc";
    // let file = "llvm_programs/complex_ifs/complex_ifs.bc";
    let path = Path::new(&file);
    dbg!(&path);
    let module = llvm_ir::Module::from_bc_path(&file).unwrap();
    // dbg!(&module.functions);

    let func = &module.functions[0];

    let (egraph, root) = parse_function(func);

    // let extractor = Extractor::new(&egraph, AstSize);
    // let (best_cost, best) = extractor.find_best(root);
    // println!("Extracted: \n{}", best.pretty(20));
    // egraph.dot().to_pdf("examples/if.pdf").unwrap();

    println!("Parsed");
    let mut initial_expr = EGraph::default();
    let root = initial_expr.add_expr(&egraph);
    initial_expr.dot().to_pdf("/tmp/parsed.pdf").unwrap();

    let runner = Runner::default()
        .with_node_limit(100000)
        // .with_time_limit(std::time::Duration::from_secs(15))
        .with_iter_limit(100)
        .with_expr(&egraph)
        .run(&rw_rules());
    println!("Runner finished");
    dbg!(runner.stop_reason.unwrap());
    // runner.egraph.dot().to_pdf("/tmp/saturated.pdf").unwrap();

    println!("Root: {:?}", &root);
    println!("Root: {:?}", &runner.roots);
    // println!("Egraph: \n {:?}", &runner.egraph);
    let extractor = Extractor::new(&runner.egraph, lang::cost_fn::NoAlloc);
    let (best_cost, best) = extractor.find_best(runner.roots[0]);

    println!("Extracted: \n{}", best.pretty(20));

    let mut extracted = EGraph::default();
    extracted.add_expr(&best);
    extracted.dot().to_pdf("/tmp/extracted.pdf").unwrap();

    // let mut egraph = EGraph::default();
    // let root = egraph.add_expr(&best);
    // egraph.dot().to_pdf("examples/if.pdf").unwrap();
    //
    // let runner = Runner::default()
    //     .with_node_limit(1000000)
    //     .with_time_limit(std::time::Duration::from_secs(15))
    //     .with_iter_limit(100)
    //     .with_egraph(egraph)
    //     .run(&rw_rules());
    // let extractor = Extractor::new(&runner.egraph, AstSize);
    // let (best_cost, best) = extractor.find_best(root);
    // println!("Extracted: \n{}", best.pretty(20));
    // runner.egraph.dot().to_pdf("examples/if.pdf").unwrap();
}
