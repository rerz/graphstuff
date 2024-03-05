use petgraph::prelude::NodeIndex;
use petgraph::Graph;
use rand::prelude::IteratorRandom;
use rand::thread_rng;
use std::collections::HashSet;

pub fn bron_kerbosch<N, E>(graph: &Graph<N, E>) -> Vec<HashSet<NodeIndex>> {
    let mut cliques = vec![];

    let mut r = HashSet::new();
    let x = HashSet::new();
    let p = graph.node_indices().collect::<HashSet<_>>();

    fn inner<N, E>(
        graph: &Graph<N, E>,
        cliques: &mut Vec<HashSet<NodeIndex>>,
        r: &mut HashSet<NodeIndex>,
        p: HashSet<NodeIndex>,
        mut x: HashSet<NodeIndex>,
    ) {
        if p.is_empty() && x.is_empty() {
            if cliques.is_empty() {
                cliques.push(r.clone());
                return;
            }

            let cur = cliques.first().unwrap().len();
            if cur < r.len() {
                cliques.clear();
            }
            if cur <= r.len() {
                cliques.push(r.clone());
            }
            return;
        }

        let pivot = *p.union(&x).choose(&mut thread_rng()).unwrap();

        let mut p_clone = p.clone();

        let neighbors = graph.neighbors(pivot).collect::<HashSet<_>>();

        for &v in p.difference(&neighbors) {
            let neighbors = graph.neighbors(v).collect::<HashSet<_>>();
            r.insert(v);
            let p1 = p_clone
                .intersection(&neighbors)
                .cloned()
                .collect::<HashSet<_>>();
            let x1 = x.intersection(&neighbors).cloned().collect::<HashSet<_>>();
            inner(graph, cliques, r, p1, x1);
            p_clone.remove(&v);
            x.insert(v);
        }
    }

    inner(graph, &mut cliques, &mut r, p, x);

    cliques
}

#[test]
fn test_graph() {
    let graph = petgraph_gen::complete_graph(10);
    bron_kerbosch(&graph);
}
