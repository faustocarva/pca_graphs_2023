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
) -> Option<HashMap<V, E>> {
    let mut distances = HashMap::with_capacity(graph.vertices_count());
    let mut prio = BinaryHeap::new();

    for vertex in graph.vertices() {
        distances.insert(*vertex, E::max_value());
    }

    prio.push(Reverse(EdgeComparator(start, start, E::default())));
    distances.insert(start, E::default());


    while let Some(Reverse(EdgeComparator(new, _, dist))) = prio.pop() {
        for (next, weight) in graph.get_adjacent_vertices(new).unwrap() {
            let next_distance = dist + *weight;
            if next_distance < *distances.get(&next).unwrap() {
                distances.insert(*next, next_distance);
                prio.push(Reverse(EdgeComparator(
                    *next,
                    new,
                    next_distance,
                )));
            }
        }
    }
    Some(distances)
}

#[cfg(test)]
mod test_mst {
    use crate::dijkstra;
    use std::collections::HashMap;

    #[test]
    fn test_dijkstra() {
        let mut graph = super::Graph::new();
        graph.add_vertex(0);
        graph.add_vertex(1);
        graph.add_vertex(2);
        graph.add_vertex(3);
        graph.add_vertex(4);        
        graph.add_edge(0, 1, 4);
        graph.add_edge(0, 2, 1);        
        graph.add_edge(2, 1, 2);
        graph.add_edge(2, 3, 5);
        graph.add_edge(1, 3, 1);
        graph.add_edge(3, 4, 3);        
        let res = dijkstra(&graph, 0);

        let hashmap: HashMap<_, _> = vec![
            (0, 0), (1, 3), (2, 1), (3, 4), (4, 7)
        ].into_iter().collect();

        assert_eq!(Some(hashmap), res);
    }
}
