mod graph;

pub use graph::Graph;

#[derive(Clone, Debug, PartialEq)]
/// Graph operation error
pub enum Error {
    /// There is no node with the given id in the graph
    NoSuchNode,

    /// There is no such edge in the graph
    NoSuchEdge,

    /// Could not add an edge to the graph
    CannotAddEdge,

    // This node already exists
    NodeAlreadyExists,

    // This edge already exists
    EdgeAlreadyExists,

    /// The operation cannot be performed as it will
    /// create a cycle in the graph.
    CycleError,
}
