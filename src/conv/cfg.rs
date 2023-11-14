use std::collections::{HashMap, VecDeque};

use llvm_ir::{Name, Terminator};

pub struct Cfg {
    pub blocks: Vec<llvm_ir::BasicBlock>,
    pub block_name_to_id: HashMap<llvm_ir::Name, usize>,
    graph: Vec<Vec<usize>>,
    transposed: Vec<Vec<usize>>,
    decision_point: Vec<Option<(usize, bool)>>,
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
                    dbg!("Reached ret");
                }
                _ => todo!("Unhandled terminator"),
            }
        }

        let mut decision_point = vec![None; blocks.len() + 1];

        dbg!(&transposed);

        let order = topo_order(&graph);
        for &node in &order {
            dbg!(&blocks[node].name);
            match &blocks[node].term {
                Terminator::Br(br) => {
                    let dest = *bname_to_id.get(&br.dest).unwrap();
                    decision_point[dest] = decision_point[node];
                }
                Terminator::CondBr(br) => {
                    let dest_true = *bname_to_id.get(&br.true_dest).unwrap();
                    let dest_false = *bname_to_id.get(&br.false_dest).unwrap();
                    decision_point[dest_true] = Some((node, true));
                    decision_point[dest_false] = Some((node, false));
                }
                Terminator::Ret(_) => {
                    dbg!("Reached ret");
                }
                _ => todo!("Unhandled terminator"),
            }
        }

        Self {
            blocks: blocks.to_vec(),
            block_name_to_id: bname_to_id,
            graph,
            transposed,
            decision_point,
        }
    }

    pub fn get_preds(&self, name: &Name) -> &[usize] {
        let &id = self.block_name_to_id.get(name).unwrap();
        &self.transposed[id]
    }

    pub fn topo_order(&self) -> Vec<usize> {
        topo_order(&self.graph)
    }

    pub fn get_decision_point(&self, name: &Name) -> (Name, (Name, Name)) {
        let &curr = self.block_name_to_id.get(name).unwrap();
        let left = self.transposed[curr][0];
        let right = self.transposed[curr][1];
        // dbg!(self.blocks[curr]);
        assert!(self.transposed[curr].len() != 1, "Not a decision");
        assert!(
            self.transposed[curr].len() <= 2,
            "Unsupported: more than 2 predecessors"
        );

        let left_dec_point = self.decision_point[left];
        let right_dec_point = self.decision_point[right];

        dbg!(&left_dec_point, &right_dec_point);

        if left_dec_point.is_none() || right_dec_point.is_none() {
            panic!("Error: decision point not found");
        }

        let (left_pred, left_cnd) = left_dec_point.unwrap();
        let (right_pred, right_cnd) = right_dec_point.unwrap();
        if left_pred != right_pred {
            dbg!(left_pred, right_pred);
            panic!("Error: not a common decision point");
        }

        if left_cnd == right_cnd {
            panic!("Error: same condition");
        }

        let left = self.blocks[left].name.clone();
        let right = self.blocks[left_pred].name.clone();

        let (true_src, false_src) = if left_cnd {
            (left, right)
        } else {
            (right, left)
        };

        (self.blocks[left_pred].name.clone(), (true_src, false_src))
    }
}
