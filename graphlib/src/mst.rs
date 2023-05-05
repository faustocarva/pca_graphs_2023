use super::{EdgeType, Graph, GraphEdgeTrait, GraphVertexTrait};
use std::cmp::Ordering;
#[allow(unused_imports)]
use union_find_rs::prelude::*;

/// Source for Minimal Spanning Tree algorithms.
/// First: Kruskal
/// Second: Prim
///
/// Kruskal (UNION/FIND DISJOINT_SET)
///     Sort edges by ascending edge weight
///     Loop over sorted edges
///     Take two vertices
///     If vertices are UNIFIED (FIND in DISJOINT_SET), don't inclued vertices
///     Else, UNION on those two edges
///     Terminate when all edges have been processed, or all vertices have been UNIFIED

pub fn kruskal<V: GraphVertexTrait, E: GraphEdgeTrait, T: EdgeType>(
    graph: &Graph<V, E, T>,
) -> Option<(E, Vec<(V, V)>)> {
    let edges = graph.edges_with_weights(Ordering::Less);
    //let mut subgraphs = UnionFind::<V>::new(edges.len());
    let mut sets: DisjointSets<V> = DisjointSets::new();
    let mut result: Vec<(V, V)> = Vec::new();
    let mut total_weight = E::default();
    //Make set
    for vettice in graph.vertices() {
        sets.make_set(*vettice).unwrap();
    }
    for edge in &edges {
        //let mut total_weight = edge.2;
        if sets.find_set(&edge.0).unwrap() != sets.find_set(&edge.1).unwrap() {
            sets.union(&edge.0, &edge.1).unwrap();
            result.push((edge.0, edge.1));
            total_weight += edge.2;
        }
    }

    if result.is_empty() {
        None
    } else {
        Some((total_weight, result))
    }
}

#[cfg(test)]
mod test_mst {
    use crate::kruskal;

    #[test]
    fn test_some_mst() {
        let mut graph = super::Graph::new();
        graph.add_vertex(1);
        graph.add_vertex(2);
        graph.add_vertex(3);
        graph.add_edge(1, 2, 5);
        graph.add_edge(2, 3, 9);
        graph.add_edge(3, 1, 1);
        let sort = kruskal(&graph);
        assert_eq!(false, sort.is_none());
    }
}
