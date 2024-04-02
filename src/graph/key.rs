use std::hash::Hash;

pub trait Key: Copy + Clone + Eq + PartialEq + Hash {}

pub trait HasKeys {
    type NodeKey: Key;
    type EdgeKey: Key;
}

pub type NK<K> = <K as HasKeys>::NodeKey;
pub type EK<K> = <K as HasKeys>::EdgeKey;
