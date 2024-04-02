use crate::backend::GraphBackend;
use crate::graph::dir::{Directed, Directionality, Undirected};
use crate::graph::Graph;
use crate::graph::key::{EK, NK};

pub trait Neighbors<D: Directionality>: Graph<Dir = D> {
    fn neighbors(&self, node: NK<Self>) -> impl Iterator<Item = NK<Self>>;

    fn in_neighbors(&self, node: NK<Self>) -> impl Iterator<Item = NK<Self>>;
}

impl<B, G> Neighbors<Directed> for G
where
    B: GraphBackend<NodeKey: Into<NK<G>>, EdgeKey: Into<EK<G>>> + 'static,
    G: Graph<Dir = Directed, Backend = B>,
    NK<G>: Into<NK<B>>,
    EK<G>: Into<EK<B>>,
{
    fn neighbors(&self, node: NK<Self>) -> impl Iterator<Item = NK<Self>> {
        self.backend()
            .out_edges(node.into())
            .map(|node| node.into())
    }

    fn in_neighbors(&self, node: NK<Self>) -> impl Iterator<Item = NK<Self>> {
        self.backend().in_edges(node.into()).map(|node| node.into())
    }
}

impl<B, G> Neighbors<Undirected> for G
where
    B: GraphBackend<NodeKey: Into<NK<G>>, EdgeKey: Into<EK<G>>> + 'static,
    G: Graph<Dir = Undirected, Backend = B>,
    NK<G>: Into<NK<B>>,
    EK<G>: Into<EK<B>>,
{
    fn neighbors(&self, node: NK<Self>) -> impl Iterator<Item = NK<Self>> {
        self.backend()
            .out_edges(node.into())
            .map(|node| node.into())
    }

    fn in_neighbors(&self, node: NK<Self>) -> impl Iterator<Item = NK<Self>> {
        self.neighbors(node)
    }
}
