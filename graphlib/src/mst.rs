use super::{EdgeComparator, EdgeType, Graph, GraphEdgeTrait, GraphVertexTrait};
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use union_find_rs::prelude::*;

/// Minimal Spanning Tree algorithms.

/// Kruskal (Union-Find over a DisjointSet)
///     Make Set using a DisjointSet for each vertex.
///     Sort edges by ascending edge weight
///     Loop over sorted edges
///     If vertices are Unified, don't inclued vertices
///     Else, Unify those two edges
///     Terminate when all edges have been processed, or all vertices have been Unified
pub fn kruskal<V: GraphVertexTrait, E: GraphEdgeTrait, T: EdgeType>(
    graph: &Graph<V, E, T>,
) -> Option<(E, Vec<(V, V)>)> {
    let edges = graph.edges_with_weights(Ordering::Less);
    let mut sets: DisjointSets<V> = DisjointSets::new();
    let mut result: Vec<(V, V)> = Vec::new();
    let mut total_weight = E::default();

    // Make set
    for vertex in graph.vertices() {
        sets.make_set(*vertex).unwrap();
    }

    // Loop over all edges in ascending sort order
    for (from, to, weight) in &edges {
        if sets.find_set(from).unwrap() != sets.find_set(to).unwrap() {
            sets.union(from, to).unwrap();
            result.push((*from, *to));
            total_weight += *weight;
        }
    }

    if result.is_empty() {
        None
    } else {
        Some((total_weight, result))
    }
}

/// Prim
pub fn prim<V: GraphVertexTrait, E: GraphEdgeTrait, T: EdgeType>(
    graph: &Graph<V, E, T>,
    start: V,
) -> Option<(E, Vec<(V, V)>)> {
    let mut prio = BinaryHeap::new();
    let mut visited: Vec<V> = Vec::new();
    let mut result: Vec<(V, V)> = Vec::new();
    let mut total_weight = E::default();

    for adjancent in graph.get_adjacent_vertices(start).unwrap_or(&vec![]) {
        prio.push(Reverse(EdgeComparator(adjancent.0, start, adjancent.1)));
    }

    visited.push(start);

    while let Some(Reverse(EdgeComparator(target, prev, dist))) = prio.pop() {
        if visited.contains(&target) {
            continue;
        }

        visited.push(target);
        result.push((prev, target));
        total_weight += dist;

        for (new_target, cost) in graph.get_adjacent_vertices(target).unwrap_or(&vec![]) {
            if !visited.contains(new_target) {
                prio.push(Reverse(EdgeComparator(*new_target, target, *cost)));
            }
        }
    }

    if visited.is_empty() {
        None
    } else {
        Some((total_weight, result))
    }
}

#[cfg(test)]
mod test_mst {
    use crate::{kruskal, prim};

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

    #[test]
    fn test_ppt_graph_mst() {
        let mut graph = super::Graph::new_undirected();
        graph.add_edge("A", "E", 1);
        graph.add_edge("I", "J", 0);
        graph.add_edge("A", "B", 5);
        graph.add_edge("C", "I", 1);
        graph.add_edge("D", "G", 11);
        graph.add_edge("C", "J", 2);
        graph.add_edge("G", "I", 4);
        graph.add_edge("D", "H", 2);
        graph.add_edge("E", "F", 1);
        graph.add_edge("D", "F", 5);
        graph.add_edge("D", "E", 2);
        graph.add_edge("A", "D", 4);
        graph.add_edge("B", "D", 2);
        graph.add_edge("C", "H", 4);
        graph.add_edge("B", "C", 4);
        graph.add_edge("H", "I", 6);
        graph.add_edge("G", "H", 1);
        graph.add_edge("F", "G", 7);
        let sort = kruskal(&graph);
        assert_eq!(14, sort.unwrap().0);
    }

    #[test]
    fn test_ppt_graph_prim() {
        let mut graph = super::Graph::new_undirected();
        graph.add_edge("A", "E", 1);
        graph.add_edge("I", "J", 0);
        graph.add_edge("A", "B", 5);
        graph.add_edge("C", "I", 1);
        graph.add_edge("D", "G", 11);
        graph.add_edge("C", "J", 2);
        graph.add_edge("G", "I", 4);
        graph.add_edge("D", "H", 2);
        graph.add_edge("E", "F", 1);
        graph.add_edge("D", "F", 5);
        graph.add_edge("D", "E", 2);
        graph.add_edge("A", "D", 4);
        graph.add_edge("B", "D", 2);
        graph.add_edge("C", "H", 4);
        graph.add_edge("B", "C", 4);
        graph.add_edge("H", "I", 6);
        graph.add_edge("G", "H", 1);
        graph.add_edge("F", "G", 7);
        let sort = prim(&graph, &"A");
        println!("{:?}", sort);
        assert_eq!(14, sort.unwrap().0);
    }
}
