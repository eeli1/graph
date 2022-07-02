use crate::{Error, Graph};
use std::fmt::Debug;

pub fn neighborhood<Node, Edge>(
    graph: Graph<Node, Edge>,
    id: usize,
    k: usize,
) -> Result<Vec<usize>, Error>
where
    Node: PartialEq + Clone + Debug,
    Edge: PartialEq + Clone + Debug,
{
    if k == 0 {
        Ok(vec![id])
    } else if k == 1 {
        graph.neighbors(id)
    } else {
        let mut visited = vec![false; graph.node_len()];
        let mut result = vec![id];
        visited[id] = true;

        for _ in 0..k {
            let mut temp = Vec::new();
            for id1 in result {
                for id2 in graph.neighbors(id1)? {
                    if !visited[id2] {
                        visited[id2] = true;
                        temp.push(id2);
                    }
                }
            }
            result = temp;
        }
        Ok(result)
    }
}
