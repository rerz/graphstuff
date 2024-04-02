use std::hash::Hash;

use crate::graph::dir::Directionality;
use crate::graph::key::{HasKeys, NK};
use crate::graph::weight::Weight;

pub mod petgraph;

pub trait MutableGraph {}

pub trait GraphBackend: HasKeys {
    type Dir: Directionality;

    type Node: Weight;
    type Edge: Weight;

    fn node_keys(&self) -> impl Iterator<Item = NK<Self>>;

    fn node_weights(&self) -> impl Iterator<Item = &Self::Node>;

    fn out_edges(&self, node: NK<Self>) -> impl Iterator<Item = NK<Self>>;

    fn in_edges(&self, node: NK<Self>) -> impl Iterator<Item = NK<Self>>;
}
