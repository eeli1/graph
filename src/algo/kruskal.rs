use crate::Graph;
use std::fmt::Debug;

struct UnionFind {
    rep: Vec<usize>,
    dep: Vec<usize>,
}

impl UnionFind {
    fn new(len: usize) -> Self {
        Self {
            rep: (0..len).collect(),
            dep: vec![1; len],
        }
    }

    fn find_set(&mut self, node_id: usize) -> usize {
        let r = self.find_rep(node_id);
        self.compress(node_id, r);
        r
    }

    fn find_rep(&mut self, node_id: usize) -> usize {
        let mut r = self.rep[node_id];
        while !self.is_rep(r) {
            r = self.rep[r];
        }
        return r;
    }


    #[allow(dead_code)]
    fn union(&mut self, node_id1: usize, node_id2: usize) -> usize {
        let r1 = self.find_rep(node_id1);
        let r2 = self.find_rep(node_id2);
        let r = self.union_sets(r1, r2);
        self.compress(node_id1, r);
        self.compress(node_id2, r);
        r
    }

    fn union_sets(&mut self, r1: usize, r2: usize) -> usize {
        if r2 == r1 {
            r1
        } else if self.dep[r1] > self.dep[r2] {
            self.rep[r2] = r1;
            r1
        } else {
            self.dep[r2] = usize::max(self.dep[r2], self.dep[r1] + 1);
            self.rep[r1] = r2;
            r2
        }
    }

    fn compress(&mut self, mut i: usize, r: usize) {
        while self.rep[i] != r {
            i = self.rep[i];
            self.rep[i] = r;
        }
    }

    fn is_rep(&self, i: usize) -> bool {
        self.rep[i] == i
    }
}

pub fn find_mst<Node: PartialEq + Clone + Debug, Edge: PartialEq + Clone + Debug>(
    graph: Graph<Node, Edge>,
    get_weight: &dyn Fn(Edge) -> usize,
) -> Vec<(usize, usize, Edge)> {
    let mut edges = graph.edges();

    edges.sort_by(|(_, _, x1), (_, _, x2)| get_weight(x1.clone()).cmp(&get_weight(x2.clone())));

    let mut result = Vec::new();
    let mut union_find = UnionFind::new(graph.node_len());

    let mut i = 0;
    while result.len() < graph.node_len() - 1 && i < edges.len() {
        let e = edges[i].clone();
        i += 1;
        let c1 = union_find.find_set(e.0);
        let c2 = union_find.find_set(e.1);
        if c1 != c2 {
            result.push(e);
            union_find.union_sets(c1, c2);
        }
    }
    return result;
}
