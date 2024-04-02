use std::collections::{HashMap, HashSet, VecDeque};

use crate::graph::dir::Directionality;
use crate::graph::Graph;
use crate::graph::key::NK;
use crate::graph::neighbors::Neighbors;

pub fn bfs<D, G>(
    graph: &G,
    root: NK<G>,
    visit: impl Fn(NK<G>) -> bool,
    mut on_next: impl FnMut(NK<G>, NK<G>),
) -> Option<NK<G>>
where
    D: Directionality,
    G: Graph<Dir = D> + Neighbors<D>,
{
    let mut queue = VecDeque::new();
    let mut explored = HashSet::new();

    explored.insert(root);
    queue.push_back(root);

    while let Some(node) = queue.pop_front() {
        let stop = visit(node);

        if stop {
            return Some(node);
        }

        for neighbor in graph.neighbors(node) {
            if explored.contains(&neighbor) {
                continue;
            }

            on_next(node, neighbor);

            explored.insert(neighbor);
            queue.push_back(neighbor);
        }
    }

    None
}

fn get_path<G: Graph>(parents: HashMap<NK<G>, NK<G>>, root: NK<G>, node: NK<G>) -> Vec<NK<G>> {
    let mut path = vec![];
    let mut current = node;
    while let Some(parent) = parents.get(&current) {
        path.push(current);
        if *parent == root {
            path.push(root);
            path.reverse();
            return path;
        }
        current = *parent;
    }

    path
}

pub fn bfs_with_path_tracking<D, G>(
    graph: &G,
    root: NK<G>,
    visit: impl Fn(NK<G>) -> bool,
) -> Option<Vec<NK<G>>>
where
    D: Directionality,
    G: Graph<Dir = D> + Neighbors<D>,
{
    let mut parents = HashMap::new();
    let on_next = |parent, next| {
        parents.insert(next, parent);
    };

    bfs(graph, root, visit, on_next).map(|node| get_path::<G>(parents, root, node))
}

pub fn bfs_with_target<D, G>(graph: &G, root: NK<G>, target: NK<G>) -> Option<Vec<NK<G>>>
where
    D: Directionality,
    G: Graph<Dir = D> + Neighbors<D>,
{
    let condition = |node| node == target;

    let path = bfs_with_path_tracking(graph, root, condition);

    path
}
