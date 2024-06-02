use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashMap;
use std::hash::Hash;

pub struct HashBackedUnGraphWithParallelEdges<N, E>
where
    N: Eq + Hash + Clone,
    E: Clone,
{
    /// Like UnGraphMap, but also support parallel edges.
    /// (UnGraphMap does not allow parallel edges, so wrapping UnGraph.)
    pub graph: UnGraph<N, E>,
    nodes: HashMap<N, NodeIndex>,
}

impl<N, E> HashBackedUnGraphWithParallelEdges<N, E>
where
    N: Eq + Hash + Clone,
    E: Clone,
{
    pub fn new() -> Self {
        Self {
            graph: UnGraph::new_undirected(),
            nodes: HashMap::<N, NodeIndex>::new(),
        }
    }

    pub fn add_edge(&mut self, first: &N, second: &N, edge: &E) {
        if !self.nodes.contains_key(first) {
            let new_node = self.graph.add_node(first.clone());
            self.nodes.insert(first.clone(), new_node);
        }
        if !self.nodes.contains_key(second) {
            let new_node = self.graph.add_node(second.clone());
            self.nodes.insert(second.clone(), new_node);
        }

        let node_a = self.nodes.get(first).unwrap();
        let node_b = self.nodes.get(second).unwrap();
        self.graph.add_edge(*node_a, *node_b, edge.clone());
    }
}
