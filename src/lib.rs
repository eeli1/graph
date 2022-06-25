#[cfg(feature = "no_std")]
use core::slice::Iter;
#[cfg(not(feature = "no_std"))]
use std::slice::Iter;

#[cfg(feature = "no_std")]
use core::fmt::Debug;
#[cfg(not(feature = "no_std"))]
use std::fmt::Debug;

#[cfg(feature = "no_std")]
use core::clone::Clone;
#[cfg(not(feature = "no_std"))]
use std::clone::Clone;

#[cfg(feature = "no_std")]
use core::cmp::PartialEq;
#[cfg(not(feature = "no_std"))]
use std::cmp::PartialEq;

#[derive(Clone, PartialEq, Debug)]
pub struct Graph<Node, Edge> {
    adj_list: Vec<Vec<(Edge, usize)>>,
    nodes: Vec<Node>,
}

impl<Node: PartialEq + Clone + Debug, Edge: PartialEq + Clone + Debug> Graph<Node, Edge> {
    pub fn new() -> Self {
        Self {
            adj_list: Vec::new(),
            nodes: Vec::new(),
        }
    }

    pub fn find_node(&self, node: Node) -> Option<usize> {
        self.nodes.iter().position(|x| x == &node)
    }

    pub fn node_iter(&self) -> Iter<Node> {
        self.nodes.iter()
    }

    pub fn nodes(&self) -> Vec<Node> {
        self.nodes.clone()
    }

    pub fn edges(&self) -> Vec<(usize, usize, Edge)> {
        let mut result = Vec::new();
        for (from, list) in self.adj_list.iter().enumerate() {
            for (edge, to) in list.clone() {
                result.push((from, to, edge.clone()));
            }
        }
        result
    }

    pub fn edge(&self, from: usize, to: usize) -> Option<Edge> {
        for (edge, i) in self.adj_list[from].clone() {
            if to == i {
                return Some(edge.clone());
            }
        }
        None
    }

    pub fn adj_list(&self) -> Vec<Vec<(Edge, usize)>> {
        self.adj_list.clone()
    }

    pub fn neighbors(&self, node_id: usize) -> Vec<(Edge, usize)> {
        assert!(node_id < self.adj_list.len());
        self.adj_list[node_id].clone()
    }

    pub fn degree(&self, node_id: usize) -> usize {
        assert!(node_id < self.adj_list.len());
        self.adj_list[node_id].len()
    }

    pub fn add_node(&mut self, node: Node) -> usize {
        assert_eq!(self.nodes.len(), self.adj_list.len());
        assert!(!self.nodes.contains(&node));
        self.nodes.push(node);
        self.adj_list.push(Vec::new());
        assert_eq!(self.nodes.len(), self.adj_list.len());
        self.nodes.len() - 1
    }

    pub fn add_edge(&mut self, from: usize, to: usize, edge: Edge) {
        assert!(from < self.nodes.len());
        assert!(to < self.nodes.len());
        assert!(self.adj_list[from].iter().position(|x| x.1 == to) == None);
        self.adj_list[from].push((edge, to));
    }

    pub fn remove_edge(&mut self, from: usize, to: usize, edge: Edge) -> bool {
        if let Some(index) = self.adj_list[from]
            .iter()
            .position(|x| (&x.0, &x.1) == (&edge, &to))
        {
            self.adj_list[from].remove(index);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
