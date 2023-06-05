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

    for page in graph.vertices() {
        pagerank.insert(*page, initial_rank);
    }

    loop {
        let mut diff = 0.0;

        for (page, _) in graph.adj_list() {
            let mut rank = 0.0;

            for incoming_pages in graph.get_incoming_vertices(*page).unwrap() {
                let linked_count = graph.get_incoming_vertices(incoming_pages).unwrap().len() as f64;
                rank += 1./linked_count * pagerank[&incoming_pages];
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
        g.add_vertex("A");
        g.add_vertex("B");
        g.add_vertex("C");
        g.add_vertex("D");        
        g.add_edge("A", "B", 0);
        g.add_edge("A", "C", 0);
        g.add_edge("A", "D", 0);        

        g.add_edge("B", "A", 0);        
        g.add_edge("B", "D", 0);        

        g.add_edge("C", "D", 0);                

        g.add_edge("D", "C", 0);                        
        g.add_edge("D", "B", 0);                                


        let rank = pagerank(&g, 0.85, 0.0000001);
        assert_eq!(rank[&"D"] > rank[&"B"], true );        
        assert_eq!(rank[&"B"] > rank[&"A"], true );                

        println!("{:?}", rank);        

    }
}