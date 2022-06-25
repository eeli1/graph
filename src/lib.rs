mod weighted_graph;

pub use weighted_graph::WeightedGraph;

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

pub trait AbstractGraph<T> {
    /// Returns true if there are no nodes, or false otherwise.
    fn is_empty(&self) -> bool;

    /// Returns the number of nodes in this graph.
    fn node_len(&self) -> usize;

    /// Returns the number of edges in this graph.
    fn edge_len(&self) -> usize;

    /// Iterates the nodes of this graph
    fn nodes(&self) -> Vec<T>;

    /// Returns true if node is a member, or false otherwise.
    fn has_node(&self, node: &T) -> bool;

    /// Iterates the neighbors of node.
    fn neighbors(&self, node_id: usize) -> Result<Vec<usize>, Error>;

    /// Returns the number of neighbors connected to node.
    fn degree(&self, node_id: usize) -> Result<usize, Error>;

    /// Returns true if an edge exists between source and target.
    fn has_edge(&self, source_id: usize, target_id: usize) -> Result<bool, Error>;

    /// Returns the adjacency list
    fn adj_list(&self) -> Vec<Vec<usize>>;

    // Returns the node corresponding to the id
    fn node(&self, node_id: usize) -> Result<T, Error>;

    // Returns the id of the node
    fn node_id(&self, node: &T) -> Result<usize, Error>;

    // Adds node and retuns node id
    fn add_node(&mut self, node: T) -> Result<usize, Error>;

    // Removes node and retuns node id
    fn remove_node(&mut self, node_id: usize) -> Result<(), Error>;
}
