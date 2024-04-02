use std::collections::HashSet;

use crate::graph::dir::Undirected;
use crate::graph::key::NK;
use crate::graph::neighbors::Neighbors;
use crate::graph::Graph;

pub struct BronKerbosch<'graph, G: Graph> {
    graph: &'graph G,
    cliques: Vec<HashSet<NK<G>>>,
}

impl<'graph, G> BronKerbosch<'graph, G>
where
    G: Neighbors<Undirected>,
{
    pub fn new(graph: &'graph G) -> Self {
        Self {
            graph,
            cliques: vec![],
        }
    }

    fn bk_without_pivot(
        &mut self,
        selection: &HashSet<NK<G>>,
        candidates: &HashSet<NK<G>>,
        excluded: &HashSet<NK<G>>,
    ) {
        if candidates.is_empty() && excluded.is_empty() {
            self.cliques.push(selection.clone());
            return;
        }

        let mut current_candidates = candidates.clone();
        let mut current_excluded = excluded.clone();

        for &u in candidates {
            let neighbors = self.graph.neighbors(u).collect::<HashSet<_>>();

            let candidates_intersect_neighbors = current_candidates
                .intersection(&neighbors)
                .cloned()
                .collect::<HashSet<_>>();
            let excluded_intersect_neighbors = current_excluded
                .intersection(&neighbors)
                .cloned()
                .collect::<HashSet<_>>();
            let selection_union_u = {
                let mut selection = selection.clone();
                selection.insert(u);
                selection
            };

            self.bk_without_pivot(
                &selection_union_u,
                &candidates_intersect_neighbors,
                &excluded_intersect_neighbors,
            );

            current_candidates.remove(&u);
            current_excluded.insert(u);
        }
    }

    pub fn run(&mut self) {
        let selection = HashSet::new();
        let candidates = self.graph.node_keys().collect::<HashSet<_>>();
        let excluded = HashSet::new();

        self.bk_without_pivot(&selection, &candidates, &excluded);
    }
}

pub fn bron_kerbosch<G>(graph: &G) -> Vec<HashSet<NK<G>>>
where
    G: Neighbors<Undirected>,
{
    let mut bk = BronKerbosch::new(graph);
    bk.run();
    bk.cliques
}
