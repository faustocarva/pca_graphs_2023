use super::{EdgeComparator, EdgeType, Graph, GraphEdgeTrait, GraphVertexTrait};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

/// Shortest path algorithms.

/// Dijkstra
/// Performs edge relaxation
/// The return could be a map/vec with the minimal value for each vertex, or
/// it could be the total cost from start to end (if end is present)
pub fn dijkstra<V: GraphVertexTrait, E: GraphEdgeTrait, T: EdgeType>(
    graph: &Graph<V, E, T>,
    start: V,
) -> Option<Vec<(V, E)>> {
    let mut distances = HashMap::with_capacity(graph.vertices_count());
    let mut visited: Vec<V> = Vec::new();
    let mut prio = BinaryHeap::new();

    for vertex in graph.vertices() {
        distances.insert(*vertex, E::max_value());
    }

    prio.push(Reverse(EdgeComparator(start, start, E::default())));
    distances.insert(start, E::default());
    visited.push(start);

    while let Some(Reverse(EdgeComparator(current_vertex, _, dist))) = prio.pop() {
        for edge in graph.get_adjacent_vertices(current_vertex).unwrap() {
            let next_distance = dist + edge.1;
            if next_distance < *distances.get(&edge.0).unwrap() {
                distances.insert(edge.0, next_distance);
                prio.push(Reverse(EdgeComparator(
                    edge.0,
                    current_vertex,
                    next_distance,
                )));
            }
        }
    }

    None
}

#[cfg(test)]
mod test_mst {
    use crate::dijkstra;

    #[test]
    fn test_dijkstra() {
        let mut graph = super::Graph::new();
        graph.add_vertex(1);
        graph.add_vertex(2);
        graph.add_vertex(3);
        graph.add_vertex(4);
        graph.add_edge(1, 2, 5);
        graph.add_edge(2, 3, 9);
        graph.add_edge(3, 4, 1);
        dijkstra(&graph, 1);
    }
}
