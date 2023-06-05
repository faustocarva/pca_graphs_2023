use super::{EdgeTypeTrait, Graph, GraphEdgeTrait, GraphVertexTrait};
use std::collections::HashMap;

/// PageRank algorithm.
/// 
pub fn pagerank<V: GraphVertexTrait, E: GraphEdgeTrait, T: EdgeTypeTrait>(
    graph: &Graph<V, E, T>, damping_factor: f64, epsilon: f64) -> HashMap<V, f64> {
    let num_pages = graph.vertices_count() as f64;
    let initial_rank = 1.0 / num_pages;

    let mut pagerank: HashMap<V, f64> = HashMap::new();
    let mut new_pagerank: HashMap<V, f64> = HashMap::new();

    // Initialize pagerank values
    for page in graph.vertices() {
        pagerank.insert(*page, initial_rank);
    }

    loop {
        let mut diff = 0.0;

        // Calculate new pagerank values
        for (page, links) in graph.adj_list() {
            let mut rank = 0.0;
            for (linked_page, _) in links {
                let linked_count = graph.get_adjacent_vertices(*linked_page).unwrap().len() as f64;
                rank += pagerank[linked_page] / linked_count;
            }
            rank = (1.0 - damping_factor) / num_pages + damping_factor * rank;
            new_pagerank.insert(*page, rank);

            diff += (rank - pagerank[page]).abs();
        }

        if diff < epsilon {
            break;
        }

        for (page, rank) in new_pagerank.iter() {
            pagerank.insert(*page, *rank);
        }

        new_pagerank.clear();
    }

    pagerank
}

#[cfg(test)]
mod test_pagerank {
    use super::pagerank;
    use super::Graph;

    #[test]
    fn test_pagerank() {
        let mut g = Graph::new();
        g.add_vertex("1");
        g.add_vertex("2");
        g.add_vertex("3");
        g.add_edge("1", "2", 0);
        g.add_edge("1", "3", 0);
        g.add_edge("2", "3", 0);
        g.add_edge("2", "1", 0);        
        g.add_edge("3", "2", 0);                
        let rank = pagerank(&g, 0.85, 0.0001);
        println!("{:?}", rank);        

    }
}