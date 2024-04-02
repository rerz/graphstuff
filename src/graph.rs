use std::marker::PhantomData;

use crate::backend::petgraph::PetgraphBackend;
use crate::backend::GraphBackend;
use crate::graph::dir::{Directed, Directionality, Undirected};
use crate::graph::key::HasKeys;
use crate::graph::mutability::{Mutability, Mutable};
use crate::graph::weight::Weight;

pub mod dir;
pub mod key;
pub mod mutability;
pub mod neighbors;
pub mod weight;

pub fn simple<M, D>(
    backend: impl GraphBackend<Dir = D>,
) -> SimpleGraph<M, D, impl GraphBackend<Dir = D>>
where
    M: Mutability,
    D: Directionality,
{
    SimpleGraph {
        backend,
        _phantom: PhantomData,
    }
}

pub trait Graph: HasKeys {
    type Dir: Directionality;

    type NodeWeight: Weight;
    type EdgeWeight: Weight;

    type Backend: GraphBackend;

    fn backend(&self) -> &Self::Backend;

    fn backend_mut(&mut self) -> &mut Self::Backend;

    fn node_keys(&self) -> impl Iterator<Item = Self::NodeKey>;

    fn node_weights(&self) -> impl Iterator<Item = &Self::NodeWeight>;
}

pub struct SimpleGraph<M, D, B> {
    backend: B,
    _phantom: PhantomData<(M, D)>,
}

impl<M, D, B> HasKeys for SimpleGraph<M, D, B>
where
    M: Mutability,
    D: Directionality,
    B: GraphBackend,
{
    type NodeKey = B::NodeKey;
    type EdgeKey = B::EdgeKey;
}

impl<M, D, B> Graph for SimpleGraph<M, D, B>
where
    M: Mutability,
    D: Directionality,
    B: GraphBackend,
{
    type Dir = D;
    type NodeWeight = B::Node;
    type EdgeWeight = B::Edge;
    type Backend = B;

    fn backend(&self) -> &Self::Backend {
        &self.backend
    }

    fn backend_mut(&mut self) -> &mut Self::Backend {
        &mut self.backend
    }

    fn node_keys(&self) -> impl Iterator<Item = Self::NodeKey> {
        self.backend.node_keys()
    }

    fn node_weights(&self) -> impl Iterator<Item = &Self::NodeWeight> {
        self.backend.node_weights()
    }
}

impl<N, E> From<petgraph::Graph<N, E, petgraph::Directed>>
    for SimpleGraph<Mutable, Directed, PetgraphBackend<petgraph::Graph<N, E, petgraph::Directed>>>
{
    fn from(value: petgraph::Graph<N, E, petgraph::Directed>) -> Self {
        SimpleGraph {
            backend: PetgraphBackend { inner: value },
            _phantom: PhantomData,
        }
    }
}

impl<N, E> From<petgraph::Graph<N, E, petgraph::Undirected>>
    for SimpleGraph<
        Mutable,
        Undirected,
        PetgraphBackend<petgraph::Graph<N, E, petgraph::Undirected>>,
    >
{
    fn from(value: petgraph::Graph<N, E, petgraph::Undirected>) -> Self {
        SimpleGraph {
            backend: PetgraphBackend { inner: value },
            _phantom: PhantomData,
        }
    }
}
