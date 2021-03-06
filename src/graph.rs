use crate::Error;

#[cfg(feature = "no_std")]
use core::fmt::Debug;
use std::collections::HashSet;
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
pub struct Graph<Node, Edge>
where
    Node: PartialEq + Clone + Debug,
    Edge: PartialEq + Clone + Debug,
{
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

    pub fn edges(&self) -> Vec<(usize, usize, Edge)> {
        let mut result = Vec::new();
        for (from, list) in self.adj_list.iter().enumerate() {
            for (edge, to) in list.clone() {
                result.push((from, to, edge.clone()));
            }
        }
        result
    }

    pub fn edge(&self, from: usize, to: usize) -> Result<Edge, Error> {
        for (edge, i) in self.adj_list[from].clone() {
            if to == i {
                return Ok(edge.clone());
            }
        }
        Err(Error::NoSuchEdge)
    }

    pub fn edge_list(&self) -> Vec<Vec<(Edge, usize)>> {
        self.adj_list.clone()
    }

    pub fn out_edges(&self, node_id: usize) -> Result<Vec<(Edge, usize)>, Error> {
        if node_id < self.nodes.len() {
            Ok(self.adj_list[node_id].clone())
        } else {
            Err(Error::NoSuchEdge)
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, edge: Edge) -> Result<(), Error> {
        if from > self.nodes.len() - 1 || to > self.nodes.len() - 1 {
            Err(Error::NoSuchNode)
        } else {
            self.adj_list[from].push((edge, to));
            Ok(())
        }
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

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn node_len(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_len(&self) -> usize {
        let mut sum = 0;
        for list in self.adj_list.clone() {
            sum += list.len();
        }
        sum
    }

    pub fn nodes(&self) -> Vec<Node> {
        self.nodes.clone()
    }

    pub fn has_node(&self, node: &Node) -> bool {
        self.nodes.contains(node)
    }

    pub fn neighbors(&self, node_id: usize) -> Result<Vec<usize>, Error> {
        if self.adj_list.len() < node_id {
            Err(Error::NoSuchNode)
        } else {
            Ok(self.adj_list[node_id]
                .iter()
                .map(|(_, i)| i.clone())
                .collect())
        }
    }

    pub fn degree(&self, node_id: usize) -> Result<usize, Error> {
        if self.adj_list.len() < node_id {
            Err(Error::NoSuchNode)
        } else {
            Ok(self.adj_list[node_id].len())
        }
    }

    pub fn has_edge(&self, source_id: usize, target_id: usize) -> Result<bool, Error> {
        if self.adj_list.len() < target_id {
            Err(Error::NoSuchNode)
        } else {
            Ok(self.neighbors(source_id)?.contains(&target_id))
        }
    }

    pub fn adj_list(&self) -> Vec<Vec<usize>> {
        let mut result = Vec::with_capacity(self.adj_list.len());
        for list in self.adj_list.clone() {
            result.push(list.iter().map(|(_, i)| i.clone()).collect());
        }

        result
    }

    pub fn node_mut(&mut self, node_id: usize) -> Result<&mut Node, Error> {
        if self.nodes.len() < node_id {
            Err(Error::NoSuchNode)
        } else {
            Ok(&mut self.nodes[node_id])
        }
    }

    pub fn node(&self, node_id: usize) -> Result<Node, Error> {
        if self.nodes.len() < node_id {
            Err(Error::NoSuchNode)
        } else {
            Ok(self.nodes[node_id].clone())
        }
    }

    pub fn node_id(&self, node: &Node) -> Result<usize, Error> {
        if let Some(i) = self.nodes.iter().position(|x| x == node) {
            Ok(i)
        } else {
            Err(Error::NoSuchNode)
        }
    }

    pub fn add_node(&mut self, node: Node) -> Result<usize, Error> {
        self.nodes.push(node);
        self.adj_list.push(Vec::new());
        Ok(self.nodes.len() - 1)
    }

    pub fn remove_node(&mut self, node_id: usize) -> Result<(), Error> {
        if self.nodes.len() < node_id {
            return Err(Error::NoSuchNode);
        }

        self.nodes.remove(node_id);
        self.adj_list.remove(node_id);

        let mut to_delete = vec![Vec::new(); self.adj_list.len()];
        for (i, list) in self.adj_list.iter_mut().enumerate() {
            for (j, (_, to)) in list.iter_mut().enumerate() {
                if to == &node_id {
                    to_delete[i].push(j);
                } else if to > &mut node_id.clone() {
                    *to -= 1;
                }
            }
        }

        for (i, list) in to_delete.iter().enumerate() {
            for &j in list {
                self.adj_list[i].remove(j);
            }
        }
        Ok(())
    }

    /// retruns `true` if graph is a [directed acyclic graph](https://en.wikipedia.org/wiki/Directed_acyclic_graph)
    pub fn is_dag(&self) -> bool {
        let mut stack = self.roots();
        if stack.len() == 0 {
            return false;
        }

        let mut visited = HashSet::new();

        while let Some(next) = stack.pop() {
            if !visited.insert(next) {
                return false;
            }
            for (_, id) in self.adj_list[next].iter() {
                stack.push(id.clone());
            }
        }
        true
    }

    /// retruns all the ids of the root nodes
    pub fn roots(&self) -> Vec<usize> {
        let mut roots = Vec::new();
        for (id, list) in self.inv_adj_list().iter().enumerate() {
            if list.len() == 0 {
                roots.push(id);
            }
        }
        roots
    }

    /// retruns all the ids of the leafs nodes
    /// in other words all nodes that have no neighbors
    pub fn leafs(&self) -> Vec<usize> {
        let mut leafs = Vec::new();
        for (id, list) in self.adj_list().iter().enumerate() {
            if list.len() == 0 {
                leafs.push(id);
            }
        }
        leafs
    }

    /// retruns the inverted adj_list
    pub fn inv_adj_list(&self) -> Vec<Vec<usize>> {
        let mut inv_adj = vec![Vec::new(); self.node_len()];

        for (from, list) in self.adj_list().iter().enumerate() {
            for to in list {
                inv_adj[to.clone()].push(from);
            }
        }

        inv_adj
    }
}
