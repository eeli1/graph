use std::fmt::Debug;
use std::{cmp::Ordering, collections::HashMap};

use crate::{Error, Graph};

struct MinHeap<T> {
    data: Vec<T>,
}

impl<T: PartialOrd + Clone> MinHeap<T> {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn insert(&mut self, node: T) {
        self.data.push(node);

        let mut i = self.data.len() - 1;
        while i > 0 {
            let child = self.data[i].clone();
            let parent = self.data[Self::parent(i)].clone();

            if parent.partial_cmp(&child) == Some(Ordering::Less) {
                return;
            }

            self.data[i] = parent;
            self.data[Self::parent(i)] = child;
            i = Self::parent(i);
        }
    }

    fn extract_min(&mut self) -> T {
        let result = self.data[0].clone();
        if self.data.len() == 1 {
            self.data.remove(0);
        } else {
            self.data[0] = self.data.remove(self.data.len() - 1);
            self.heapify();
        }

        return result;
    }

    fn heapify(&mut self) {
        let mut i = 0;

        loop {
            if Self::left_child(i) >= self.data.len() {
                return;
            }

            let parent = self.data[i].clone();
            let left = self.data[Self::left_child(i)].clone();
            let right;
            let mut is_left = parent.partial_cmp(&left) == Some(Ordering::Less);
            let is_right;

            if Self::right_child(i) < self.data.len() {
                right = Some(self.data[Self::right_child(i)].clone());
                is_right = parent.partial_cmp(&right.clone().unwrap()) == Some(Ordering::Less);
            } else {
                right = None;
                is_right = false;
            }

            if !is_left && !is_right {
                return;
            }

            if is_left && is_right {
                if left.partial_cmp(&right.clone().unwrap()) != Some(Ordering::Greater) {
                    is_left = false;
                }
            }

            if is_left {
                self.data[i] = left;
                self.data[Self::left_child(i)] = parent;
                i = Self::left_child(i);
            } else {
                self.data[i] = right.unwrap();
                self.data[Self::right_child(i)] = parent;
                i = Self::right_child(i);
            }
        }
    }

    fn left_child(pos: usize) -> usize {
        2 * pos + 1
    }

    fn right_child(pos: usize) -> usize {
        2 * pos + 2
    }

    fn parent(pos: usize) -> usize {
        (pos - 1) / 2
    }
}

pub fn dijkstra<Node: PartialEq + Clone + Debug, Edge: PartialEq + Clone + Debug>(
    graph: Graph<Node, Edge>,
    start_id: usize,
    end_id: usize,
    get_weight: &dyn Fn(Edge) -> usize,
) -> Result<HashMap<usize, usize>, Error> {
    let mut dist = HashMap::new(); // = vec![usize::MAX; graph.node_len()];

    let mut min_heap = MinHeap::new();

    dist.insert(start_id, 0);
    min_heap.insert((start_id, 0));

    while !min_heap.is_empty() {
        let id = min_heap.extract_min().0;
        if id == end_id {
            return Ok(dist);
        }

        for (edge, end_id) in graph.out_edges(id)? {
            let new_dist = dist.get(&id).unwrap() + get_weight(edge);
            if &new_dist < dist.get(&end_id).unwrap() {
                dist.insert(end_id, new_dist);
                min_heap.insert((end_id, new_dist));
            }
        }
    }
    return Err(Error::NoSuchPath);
}
