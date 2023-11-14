use lang::conv::to_epeg::parse_function;
use std::env;
use std::io::stdin;
use std::path::Path;

fn main() {
    let file = "llvm_programs/if/if.bc";
    let path = Path::new(file);
    dbg!(&path);
    let module = llvm_ir::Module::from_bc_path(path).unwrap();
    dbg!(&module.functions);

    let func = &module.functions[0];

    let egraph = parse_function(func);
    dbg!(&egraph);
}
