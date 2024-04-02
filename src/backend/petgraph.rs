use petgraph::graph::DefaultIx;
use petgraph::prelude::NodeIndex;
use petgraph::Graph;

use crate::backend::GraphBackend;
use crate::graph::dir::Directed;
use crate::graph::key::{HasKeys, Key, NK};
use crate::graph::weight::Weight;

impl Key for u32 {}

pub fn two_triangles() -> petgraph::Graph<(), (), petgraph::Undirected> {
    Graph::<(), (), petgraph::Undirected>::from_edges([
        (0u32, 1),
        (1, 2),
        (2, 0),
        (2, 3),
        (3, 4),
        (4, 5),
        (5, 3),
    ])
}

pub struct PetgraphBackend<T> {
    pub inner: T,
}

impl<T> PetgraphBackend<T> {
    pub fn new(inner: T) -> PetgraphBackend<T> {
        Self { inner }
    }
}

impl<N, E> HasKeys for PetgraphBackend<petgraph::Graph<N, E, petgraph::Directed>> {
    type NodeKey = DefaultIx;
    type EdgeKey = DefaultIx;
}

impl<N, E> GraphBackend for PetgraphBackend<petgraph::Graph<N, E, petgraph::Directed>>
where
    N: Weight,
    E: Weight,
{
    type Dir = Directed;
    type Node = N;
    type Edge = E;

    fn node_keys(&self) -> impl Iterator<Item = NK<Self>> {
        self.inner.node_indices().map(|idx| idx.index() as u32)
    }

    fn node_weights(&self) -> impl Iterator<Item = &Self::Node> {
        self.inner.node_weights()
    }

    fn out_edges(&self, node: NK<Self>) -> impl Iterator<Item = NK<Self>> {
        self.inner
            .neighbors(NodeIndex::new(node as usize))
            .map(|idx| idx.index() as u32)
    }

    fn in_edges(&self, node: NK<Self>) -> impl Iterator<Item = NK<Self>> {
        self.inner
            .neighbors_directed(NodeIndex::new(node as usize), petgraph::Direction::Incoming)
            .map(|idx| idx.index() as u32)
    }
}

impl<N, E> HasKeys for PetgraphBackend<petgraph::Graph<N, E, petgraph::Undirected>> {
    type NodeKey = DefaultIx;
    type EdgeKey = DefaultIx;
}

impl<N, E> GraphBackend for PetgraphBackend<petgraph::Graph<N, E, petgraph::Undirected>>
where
    N: Weight,
    E: Weight,
{
    type Dir = Directed;
    type Node = N;
    type Edge = E;

    fn node_keys(&self) -> impl Iterator<Item = NK<Self>> {
        self.inner.node_indices().map(|idx| idx.index() as u32)
    }

    fn node_weights(&self) -> impl Iterator<Item = &Self::Node> {
        self.inner.node_weights()
    }

    fn out_edges(&self, node: NK<Self>) -> impl Iterator<Item = NK<Self>> {
        self.inner
            .neighbors(NodeIndex::new(node as usize))
            .map(|idx| idx.index() as u32)
    }

    fn in_edges(&self, node: NK<Self>) -> impl Iterator<Item = NK<Self>> {
        self.out_edges(node)
    }
}
