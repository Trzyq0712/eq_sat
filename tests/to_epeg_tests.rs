use ::lang::conv::to_epeg;
use ::lang::interp;
use ::lang::rules::rw_rules;
use egg::{AstDepth, Extractor, Runner};
use llvm_ir::Module;

#[test]
fn add_conversion() {
    let module = Module::from_bc_path("llvm_programs/add/add.bc").unwrap();
    let add_func = module.get_func_by_name("add").unwrap();
    let _params = &add_func.parameters;
    let mut env = interp::Env::default();
    env.set("0".into(), interp::Value::I64(3));
    env.set("1".into(), interp::Value::I64(4));
    let (expr_og, root) = to_epeg::parse_function(add_func);


    let expr = interp::Expr::with_root(&expr_og, root);
    let res = expr.interp(&env, &mut interp::Store::default());
    assert_eq!(res, Ok(interp::Value::I64(7)));

    let runner = Runner::default()
        .with_node_limit(1000000)
        .with_time_limit(std::time::Duration::from_secs(15))
        .with_iter_limit(100)
        .with_expr(&expr_og)
        .run(&rw_rules());

    let extractor = Extractor::new(&runner.egraph, AstDepth);
    let (_, best) = extractor.find_best(root);

    let res = interp::Expr::new(&best).interp(&env, &mut interp::Store::default());

    assert_eq!(res, Ok(interp::Value::I64(7)));
}

