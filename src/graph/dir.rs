use crate::graph::Graph;

pub type Dir<G> = <G as Graph>::Dir;

pub trait Directionality {
    const IS_DIRECTED: bool;
}

pub struct Directed;

pub trait DirectedGraph {}

impl<G> DirectedGraph for G where G: Graph<Dir = Directed> {}

impl Directionality for Directed {
    const IS_DIRECTED: bool = true;
}

pub struct Undirected;

pub trait UndirectedGraph {}

impl<G> UndirectedGraph for G where G: Graph<Dir = Undirected> {}

impl Directionality for Undirected {
    const IS_DIRECTED: bool = false;
}
