use graph::Graph;

#[derive(Clone, PartialEq, Debug)]
struct Node {
    x: usize,
    y: usize,
}

#[derive(Clone, PartialEq, Debug)]
struct Edge {
    weight: usize,
}

#[test]
fn crate_graph() {
    let mut graph = Graph::new();

    let id1 = graph.add_node(Node { x: 1, y: 1 }).unwrap();
    let id2 = graph.add_node(Node { x: 2, y: 2 }).unwrap();
    let id3 = graph.add_node(Node { x: 3, y: 3 }).unwrap();

    graph.add_edge(id1, id2, Edge { weight: 1 }).unwrap();
    graph.add_edge(id1, id3, Edge { weight: 1 }).unwrap();
    graph.add_edge(id3, id1, Edge { weight: 1 }).unwrap();
}
