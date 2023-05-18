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
            if next_distance < *distances.get(next).unwrap() {
                distances.insert(*next, next_distance);
                prio.push(Reverse(EdgeComparator(*next, new, next_distance)));
            }
        }
    }
    Some(distances)
}

#[allow(unused_variables)]
#[allow(dead_code)]
/// Bellman-Ford
pub fn bellman_ford<V: GraphVertexTrait, E: GraphEdgeTrait, T: EdgeType>(
    graph: &Graph<V, E, T>,
    start: V,
) -> Option<HashMap<V, E>> {
    None
}

#[cfg(test)]
mod test_mst {
    use crate::dijkstra;
    use std::collections::HashMap;

    #[test]
    fn test_single_edge() {
        let mut graph = super::Graph::new();
        graph.add_edge(0, 1, 2);

        let dists_0: HashMap<_, _> = vec![(0, 0), (1, 2)].into_iter().collect();

        assert_eq!(dijkstra(&graph, 0), Some(dists_0));

        let dists_1: HashMap<_, _> = vec![(1, 0), (0, i32::max_value())].into_iter().collect();

        assert_eq!(dijkstra(&graph, 1), Some(dists_1));
    }

    #[test]
    fn test_negative_cycle() {}

    #[test]
    fn test_cities_graph() {
        let mut graph = super::Graph::new_undirected();
        graph.add_edge("New York", "Pittsburgh", 400);
        graph.add_edge("New York", "Philadelphia", 100);
        graph.add_edge("Pittsburgh", "Indianapolis", 400);
        graph.add_edge("Pittsburgh", "Columbus", 185);
        graph.add_edge("Philadelphia", "Columbus", 450);
        graph.add_edge("Philadelphia", "Washington, D.C.", 140);
        graph.add_edge("Indianapolis", "St. Louis", 250);
        graph.add_edge("Indianapolis", "Nashville", 290);
        graph.add_edge("Columbus", "St. Louis", 540);
        graph.add_edge("Columbus", "Chicago", 360);
        graph.add_edge("Washington, D.C.", "Nashville", 650);
        graph.add_edge("Washington, D.C.", "Charlotte", 400);
        graph.add_edge("Nashville", "Oklahoma City", 690);
        graph.add_edge("St. Louis", "Oklahoma City", 500);
        graph.add_edge("St. Louis", "Kansas City", 250);
        graph.add_edge("Chicago", "Kansas City", 500);
        graph.add_edge("Chicago", "Denver", 1000);
        graph.add_edge("Charlotte", "Atlanta", 250);
        graph.add_edge("Atlanta", "Dallas", 780);
        graph.add_edge("Atlanta", "New Orleans", 470);
        graph.add_edge("Denver", "Albuquerque", 450);
        graph.add_edge("Denver", "Las Vegas", 750);
        graph.add_edge("Kansas City", "Albuquerque", 800);
        graph.add_edge("Oklahoma City", "Albuquerque", 550);
        graph.add_edge("Oklahoma City", "Amarillo", 260);
        graph.add_edge("Dallas", "Phoenix", 1000);
        graph.add_edge("Dallas", "San Antonio", 275);
        graph.add_edge("New Orleans", "Houston", 350);
        graph.add_edge("New Orleans", "San Antonio", 540);
        graph.add_edge("Albuquerque", "Phoenix", 400);
        graph.add_edge("Albuquerque", "Tucson", 320);
        graph.add_edge("Las Vegas", "Phoenix", 300);
        graph.add_edge("Las Vegas", "Los Angeles", 270);
        graph.add_edge("San Antonio", "Phoenix", 850);
        graph.add_edge("San Antonio", "El Paso", 550);
        graph.add_edge("Houston", "El Paso", 750);
        graph.add_edge("Charlotte", "Miami", 650);
        graph.add_edge("Miami", "Tampa", 280);
        graph.add_edge("Miami", "Orlando", 230);
        graph.add_edge("Miami", "Jacksonville", 350);

        let nyc = dijkstra(&graph, "New York").unwrap();
        println!("{:?}", nyc);
        assert_eq!(Some(&1290), nyc.get("Miami"));
        assert_eq!(Some(&0), nyc.get("New York"));

        let denver = dijkstra(&graph, "Denver").unwrap();
        println!("{:?}", denver);
        assert_eq!(Some(&750), denver.get("Las Vegas"));
        assert_eq!(Some(&0), denver.get("Denver"));

        let pits = dijkstra(&graph, "Pittsburgh").unwrap();
        println!("{:?}", pits);
        assert_eq!(Some(&400), pits.get("New York"));
        assert_ne!(Some(&775), pits.get("Washington, D.C."));
        assert_eq!(Some(&640), pits.get("Washington, D.C."));
    }

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

        let hashmap: HashMap<_, _> = vec![(0, 0), (1, 3), (2, 1), (3, 4), (4, 7)]
            .into_iter()
            .collect();

        assert_eq!(Some(hashmap), res);
    }
}
