use std::collections::HashSet;

use llvm_ir_analysis::{CFGNode, ControlFlowGraph, DominatorTree};

#[derive(Debug, Clone)]
struct Loop {
    header: llvm_ir::Name,
    back_node: llvm_ir::Name,
    exit_node: llvm_ir::Name,
    parts: Vec<llvm_ir::Name>,
}

/// Finds all natural loops in the given function
fn find_loops(module: &llvm_ir::Module, func: &str) -> Vec<Loop> {
    let analysis = llvm_ir_analysis::ModuleAnalysis::new(module);
    let fn_analysis = analysis.fn_analysis(func);
    let cfg = fn_analysis.control_flow_graph();
    let dom_tree = fn_analysis.dominator_tree();

    let func = module.get_func_by_name(func).unwrap();

    let mut loops = vec![];

    for header in &func.basic_blocks {
        let back_nodes = cfg
            .preds(&header.name)
            .filter(|node| dom_tree.dominates(CFGNode::Block(&header.name), CFGNode::Block(node)))
            .collect::<Vec<_>>();
        if !back_nodes.is_empty() {
            assert!(
                back_nodes.len() == 1,
                "Only single back edge loops are supported"
            );
            let loop_nodes = find_loop_nodes(&header.name, back_nodes.clone(), &cfg, &dom_tree);
            let exit_node = loop_nodes
                .iter()
                .filter(|node| {
                    cfg.succs(node)
                        .any(|succ| !loop_nodes.iter().map(CFGNode::Block).any(|n| n == succ))
                })
                .collect::<Vec<_>>();
            if exit_node.len() != 1 {
                panic!("No support for multiple exit nodes in a loop");
            }
            let exit_node = exit_node[0].clone();
            loops.push(Loop {
                header: header.name.clone(),
                back_node: back_nodes[0].clone(),
                parts: loop_nodes,
                exit_node,
            });
        }
    }

    loops
}

fn find_loop_nodes(
    header: &llvm_ir::Name,
    back_nodes: Vec<&llvm_ir::Name>,
    cfg: &ControlFlowGraph<'_>,
    dom_tree: &DominatorTree<'_>,
) -> Vec<llvm_ir::Name> {
    let mut worklist = back_nodes;
    let mut seen = worklist.iter().cloned().collect::<HashSet<_>>();

    while let Some(node) = worklist.pop() {
        for pred in cfg.preds(node) {
            // To be part of the natural loop, the predecessor must be strictly dominated by the header
            // Also only consider node if it hasn't been seen before
            if dom_tree.strictly_dominates(CFGNode::Block(header), CFGNode::Block(pred))
                && seen.insert(pred)
            {
                worklist.push(pred);
            }
        }
    }

    seen.into_iter()
        .cloned()
        .chain(std::iter::once(header.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertor::*;
    use llvm_ir::{Module, Name};

    #[test]
    fn while_add_hand() {
        let module =
            Module::from_ir_path("llvm_programs/while_add_hand/while_add_hand.ll").unwrap();
        let func_name = "while_add_hand";
        let loops = find_loops(&module, func_name);
        assert_that!(loops).has_length(1);
        assert_that!(loops[0].header).is_equal_to(Name::from("loop.head"));
        assert_that!(loops[0].back_node).is_equal_to(Name::from("loop.body"));
        assert_that!(loops[0].exit_node).is_equal_to(Name::from("loop.head"));
        assert_that!(loops[0].parts)
            .contains_exactly(vec![Name::from("loop.head"), Name::from("loop.body")]);
    }

    #[test]
    fn double_loop() {
        let module = Module::from_ir_path("llvm_programs/double_loop/double_loop.ll").unwrap();
        let func_name = "double_loop";
        let loops = find_loops(&module, func_name);
        assert_that!(loops).has_length(2);

        let l1 = &loops[0];
        assert_that!(l1.header).is_equal_to(Name::from(6));
        assert_that!(l1.back_node).is_equal_to(Name::from(25));
        assert_that!(l1.exit_node).is_equal_to(Name::from(6));
        assert_that!(l1.parts).contains_exactly(
            [6, 10, 11, 15, 21, 24, 25]
                .into_iter()
                .map(Name::from)
                .collect::<Vec<_>>(),
        );

        let l2 = &loops[1];
        assert_that!(l2.header).is_equal_to(Name::from(11));
        assert_that!(l2.back_node).is_equal_to(Name::from(21));
        assert_that!(l2.exit_node).is_equal_to(Name::from(11));
        assert_that!(l2.parts)
            .contains_exactly([11, 15, 21].into_iter().map(Name::from).collect::<Vec<_>>());
    }
}
