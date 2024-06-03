use petgraph::dot::Dot;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::prelude::EdgeRef;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
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

    pub fn add_node(&mut self, node: &N) -> NodeIndex {
        // let node_index = self.nodes.entry(node).or_insert_with(|| value)
        if let Some(node_index) = self.nodes.get(node) {
            *node_index
        } else {
            let new_node = self.graph.add_node(node.clone());
            self.nodes.insert(node.clone(), new_node);
            new_node
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

    pub fn neighbors(&self, node: &N) -> Self {
        let mut subgraph = Self::new();
        if let Some(node_index) = self.nodes.get(node) {
            for neighbor in self.graph.neighbors(*node_index) {
                // The same neighbor is repeated for every parallel edge, so we guard any follow-up
                // addition with a membership check, to avoid duplicates.
                if !subgraph
                    .nodes
                    .contains_key(self.graph.node_weight(neighbor).unwrap())
                {
                    for edge in self.graph.edges_connecting(*node_index, neighbor) {
                        subgraph.add_edge(
                            self.graph.node_weight(*node_index).unwrap(),
                            self.graph.node_weight(neighbor).unwrap(),
                            edge.weight(),
                        );
                    }
                }
            }
        }
        subgraph
    }
}

impl<N, E> PartialEq for HashBackedUnGraphWithParallelEdges<N, E>
where
    N: Eq + Hash + Clone + std::fmt::Display + Ord,
    E: Clone + std::fmt::Display,
{
    fn eq(&self, other: &Self) -> bool {
        // FIXME shouldn't be based on mermaid
        self.graph.to_mermaid() == other.graph.to_mermaid()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbors() {
        let mut g = HashBackedUnGraphWithParallelEdges::<String, String>::new();
        g.add_node(&"Yo".to_string());
        g.add_edge(
            &String::from("A"),
            &String::from("B1"),
            &String::from("A-B1 first"),
        );
        g.add_edge(
            &String::from("A"),
            &String::from("B1"),
            &String::from("A-B1 second"),
        );
        g.add_edge(
            &String::from("A"),
            &String::from("B2"),
            &String::from("A-B2"),
        );

        // Create the expected neighbors subgraph
        let mut h = HashBackedUnGraphWithParallelEdges::<String, String>::new();
        h.add_edge(
            &String::from("A"),
            &String::from("B2"),
            &String::from("A-B2"),
        );
        h.add_edge(
            &String::from("B1"),
            &String::from("A"),
            &String::from("A-B1 second"),
        );
        h.add_edge(
            &String::from("B1"),
            &String::from("A"),
            &String::from("A-B1 first"),
        );
        assert_eq!(g.neighbors(&String::from("A")), h);
    }
}

pub trait GraphAsCode {
    fn to_graphviz(&self) -> String;
    fn to_mermaid(&self) -> String;
}

impl<N: std::fmt::Display + Ord, E: std::fmt::Display> GraphAsCode for UnGraph<N, E> {
    fn to_graphviz(&self) -> String {
        format!("{}", Dot::new(&self))
    }

    fn to_mermaid(&self) -> String {
        let mut output = String::new();

        // FIXME iterating over edges does not take into account apps without any relations
        //  Need to iterate by nodes instead.
        for e in self.edge_references() {
            let label = e.weight();
            let first = self.node_weight(e.source()).unwrap();
            let second = self.node_weight(e.target()).unwrap();

            let (first, second) = (std::cmp::min(first, second), std::cmp::max(first, second));

            output.push_str(&format!("{} ---|{}| {}\n", first, label, second));
        }

        format!("graph LR\n{}", output)
    }
}
