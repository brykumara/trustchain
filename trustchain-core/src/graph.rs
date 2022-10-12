use petgraph::adj::NodeIndex;
use petgraph::dot::{Config, Dot};
use petgraph::graph::DiGraph;
use ssi::did::Document;
use std::fmt::Display;
use thiserror::Error;

/// An error relating to Trustchain graphs.
#[derive(Error, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GraphError {
    /// Constructed graph is not a tree.
    #[error("Graph is not a tree.")]
    NotATree,
    // TODO add error types
}

/// Wrapper struct for a petgraph DiGraph of documents.
#[derive(Debug)]
struct TrustchainGraph {
    // TODO: check this is correct type spec
    graph: DiGraph<Document, Document>,
}

/// Read trees from a vector of vectors (list of trees) and return a DiGraph.
/// See: https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html
fn read_trees(trees: &Vec<Vec<Document>>) -> DiGraph<Document, Document> {
    let mut graph = DiGraph::new();
    let mut edges = Vec::new();
    for tree in trees {
        let ns: Option<u32> = None;
        let nt: Option<u32> = None;
        for i in 1..tree.len() {
            let ns = graph.add_node(tree[i - 1].clone());
            let nt = graph.add_node(tree[i].clone());
            edges.push((ns, nt));
        }
    }
    graph.extend_with_edges(&edges);
    graph
}

impl TrustchainGraph {
    /// Makes a new TrustchainGraph instance.
    fn new(trees: &Vec<Vec<Document>>) -> Result<Self, GraphError> {
        let graph = read_trees(&trees);
        Ok(Self { graph })
    }

    /// Outputs graph to graphviz format.
    fn to_graphviz(&self) {
        todo!()
    }
}

impl Display for TrustchainGraph {
    /// TODO: Implements diplay.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{
        TEST_SIDETREE_DOCUMENT, TEST_SIDETREE_DOCUMENT_METADATA,
        TEST_SIDETREE_DOCUMENT_MULTIPLE_PROOF, TEST_SIDETREE_DOCUMENT_SERVICE_AND_PROOF,
        TEST_SIDETREE_DOCUMENT_SERVICE_NOT_PROOF, TEST_SIDETREE_DOCUMENT_WITH_CONTROLLER,
        TEST_TRUSTCHAIN_DOCUMENT, TEST_TRUSTCHAIN_DOCUMENT_METADATA,
    };
    use crate::utils::canonicalize;

    #[test]
    fn read_trees() {
        let doc1: Document = serde_json::from_str(TEST_TRUSTCHAIN_DOCUMENT).unwrap();
        let doc2: Document = serde_json::from_str(TEST_TRUSTCHAIN_DOCUMENT).unwrap();
        let mut trees = Vec::new();
        trees.push(vec![doc1, doc2]);
        let graph = TrustchainGraph::new(&trees).unwrap();
        println!("{:?}", graph);

        // Output the tree to `graphviz` `DOT` format
        // println!("{:?}", Dot::with_config(&graph, &[Config::NodeIndexLabel]));
        // println!("{:?}", Dot::with_config(&graph, &[Config::NodeIndexLabel]));
        // todo!()
    }
    #[test]
    fn invalid_not_a_tree() {
        todo!()
    }
    #[test]
    fn valid_tree() {
        todo!()
    }
    #[test]
    fn to_graphviz() {
        todo!()
    }
    #[test]
    fn display() {
        todo!()
    }
}
