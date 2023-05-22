use super::{EdgeTypeTrait, Graph, GraphEdgeTrait, GraphVertexTrait};
use std::collections::BTreeMap;

/// All-Pairs Shortest Path algorithms.

/// Floyd-Warshall algorithm
pub fn floyd_warshall<V: GraphVertexTrait, E: GraphEdgeTrait, T: EdgeTypeTrait>(
    graph: &Graph<V, E, T>,
) -> Option<BTreeMap<V, BTreeMap<V, E>>> {
    let mut weight_matrix: BTreeMap<V, BTreeMap<V, E>> = BTreeMap::new(); // |V|x|V| matrix

    // Build a weight matrix from input graph
    for u in graph.vertices() {
        weight_matrix.insert(*u, BTreeMap::new());
        for v in graph.vertices() {
            if u == v {
                weight_matrix
                    .entry(*u)
                    .or_default()
                    .insert(*v, E::default());
            } else {
                weight_matrix
                    .entry(*u)
                    .or_default()
                    .insert(*v, E::max_value());
            }
        }
    }
    // update distances already known
    for (u, v, weight) in graph.edges() {
        weight_matrix.entry(u).or_default().insert(v, weight);
    }

    let keys = weight_matrix.keys().copied().collect::<Vec<_>>();

    for &k in &keys {
        for &i in &keys {
            for &j in &keys {
                let ij = weight_matrix[&i][&j];
                let ik = weight_matrix[&i][&k];
                let kj = weight_matrix[&k][&j];
                let result = ik.checked_add(&kj);
                if let Some(sum) = result {
                    if ij > sum {
                        weight_matrix.entry(i).or_default().insert(j, sum);
                    }
                }
            }
        }
    }

    for &i in &keys {
        if weight_matrix[&i][&i] < E::default() {
            return None;
        }
    }

    Some(weight_matrix)
}

#[cfg(test)]
mod test_single_path {
    use super::floyd_warshall;
    use std::collections::BTreeMap;

    #[test]
    fn test_single_edge() {
        let mut graph = super::Graph::new();
        graph.add_edge("a", "d", 60);
        graph.add_edge("a", "c", 12);
        graph.add_edge("c", "b", 20);
        graph.add_edge("b", "a", 10);

        let mut dists_0 = BTreeMap::new();
        dists_0.insert("a", BTreeMap::new());
        dists_0.insert("b", BTreeMap::new());
        dists_0.insert("c", BTreeMap::new());
        dists_0.insert("d", BTreeMap::new());

        dists_0.get_mut(&"a").unwrap().insert("a", 0);
        dists_0.get_mut(&"a").unwrap().insert("b", 32);
        dists_0.get_mut(&"a").unwrap().insert("c", 12);
        dists_0.get_mut(&"a").unwrap().insert("d", 60);

        dists_0.get_mut(&"b").unwrap().insert("a", 10);
        dists_0.get_mut(&"b").unwrap().insert("b", 0);
        dists_0.get_mut(&"b").unwrap().insert("c", 22);
        dists_0.get_mut(&"b").unwrap().insert("d", 70);

        dists_0.get_mut(&"c").unwrap().insert("a", 30);
        dists_0.get_mut(&"c").unwrap().insert("b", 20);
        dists_0.get_mut(&"c").unwrap().insert("c", 0);
        dists_0.get_mut(&"c").unwrap().insert("d", 90);

        dists_0.get_mut(&"d").unwrap().insert("a", i32::max_value());
        dists_0.get_mut(&"d").unwrap().insert("b", i32::max_value());
        dists_0.get_mut(&"d").unwrap().insert("c", i32::max_value());
        dists_0.get_mut(&"d").unwrap().insert("d", 0);

        let res = floyd_warshall(&graph);
        println!("{:?}", res);
        assert_eq!(dists_0, res.unwrap());
    }

    #[test]
    fn test_detect_negative_cycle() {
        let mut graph = super::Graph::new();
        graph.add_edge(1, 2, 1);
        graph.add_edge(2, 3, 3);
        graph.add_edge(3, 4, 2);
        graph.add_edge(4, 2, -6);
        assert_eq!(None, floyd_warshall(&graph));
    }
}
