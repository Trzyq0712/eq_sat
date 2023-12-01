use std::collections::{HashMap, VecDeque};

use llvm_ir::{Name, Terminator};

pub struct Cfg {
    pub blocks: Vec<llvm_ir::BasicBlock>,
    lbl_to_id: HashMap<llvm_ir::Name, usize>,
    graph: Vec<Vec<usize>>,
    transposed: Vec<Vec<usize>>,
    ret_blocks: Vec<usize>,
}

fn topo_order(graph: &[Vec<usize>]) -> Vec<usize> {
    let mut incoming = vec![0; graph.len()];
    for node in graph {
        for &dest in node {
            incoming[dest] += 1;
        }
    }

    let mut queue = VecDeque::from_iter(
        incoming
            .iter()
            .enumerate()
            .filter_map(|(i, &x)| (x == 0).then_some(i)),
    );

    let mut order = Vec::new();
    while let Some(x) = queue.pop_front() {
        order.push(x);
        for &dest in &graph[x] {
            incoming[dest] -= 1;
            if incoming[dest] == 0 {
                queue.push_back(dest);
            }
        }
    }

    order
}

impl Cfg {
    pub fn new(blocks: &[llvm_ir::BasicBlock]) -> Self {
        let mut bname_to_id = HashMap::new();
        let mut graph = vec![Vec::new(); blocks.len()];
        let mut transposed = vec![Vec::new(); blocks.len()];
        let mut ret_blocks = Vec::new();
        for (i, block) in blocks.iter().enumerate() {
            bname_to_id.insert(block.name.clone(), i);
        }
        let exit_id = blocks.len();
        for (i, block) in blocks.iter().enumerate() {
            match &block.term {
                Terminator::Br(br) => {
                    let dest = *bname_to_id.get(&br.dest).unwrap();
                    graph[i].push(dest);
                    transposed[dest].push(i);
                }
                Terminator::CondBr(condBr) => {
                    let dest_true = *bname_to_id.get(&condBr.true_dest).unwrap();
                    let dest_false = *bname_to_id.get(&condBr.false_dest).unwrap();
                    graph[i].extend_from_slice(&[dest_true, dest_false]);
                    transposed[dest_true].push(i);
                    transposed[dest_false].push(i);
                }
                Terminator::Ret(_) => {
                    ret_blocks.push(i);
                }
                _ => todo!("Unhandled terminator"),
            }
        }

        Self {
            blocks: blocks.to_vec(),
            lbl_to_id: bname_to_id,
            graph,
            transposed,
            ret_blocks,
        }
    }

    pub fn ret_blocks(&self) -> &[usize] {
        &self.ret_blocks
    }

    pub fn id_of(&self, name: &Name) -> usize {
        *self.lbl_to_id.get(name).unwrap()
    }

    pub fn succs(&self, name: &Name) -> &[usize] {
        let &id = self.lbl_to_id.get(name).unwrap();
        &self.graph[id]
    }

    pub fn preds(&self, name: &Name) -> &[usize] {
        let &id = self.lbl_to_id.get(name).unwrap();
        &self.transposed[id]
    }

    pub fn topo_order(&self) -> Vec<usize> {
        topo_order(&self.graph)
    }
}
