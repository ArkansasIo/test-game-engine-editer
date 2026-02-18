//! Graph utilities (nodes, edges, pathfinding)
use std::collections::HashMap;

pub struct Graph<N, E> {
    pub nodes: HashMap<usize, N>,
    pub edges: HashMap<(usize, usize), E>,
}

impl<N, E> Graph<N, E> {
    pub fn new() -> Self {
        Self { nodes: HashMap::new(), edges: HashMap::new() }
    }
    pub fn add_node(&mut self, id: usize, node: N) {
        self.nodes.insert(id, node);
    }
    pub fn add_edge(&mut self, from: usize, to: usize, edge: E) {
        self.edges.insert((from, to), edge);
    }
}
