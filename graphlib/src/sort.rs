use std::collections::HashMap;
use std::collections::VecDeque;
use super::{Graph, GraphElemTrait};


/// Performs topological sort using the Kahn's algorithm.
/// Returns a Vec storing the vertices in a the topological order.
pub fn topological_sort<V: GraphElemTrait, E: GraphElemTrait>(graph: &Graph<V, E>) -> Option<Vec<V>> {
    // 1) Preparation:
    //  Build a map of vertices with incoming edges count
    //  Add vertices that have no incoming edges to a queue
    let mut incoming_edges_count: HashMap<V, usize> = HashMap::new();
    for elem in graph.edges() {
        incoming_edges_count.entry(elem.0).or_insert(0);
        *incoming_edges_count.entry(elem.1).or_insert(0) += 1;
    }
    let mut no_incoming_edges: VecDeque<V> = VecDeque::new();
    for (node, count) in &incoming_edges_count {
        if *count == 0 {
            no_incoming_edges.push_back(*node);
        }
    }

    // 2) Kahn's algorithm:
    //  For each node in the no incoming edge queue
    //      Remove from queue, add to the sort vector
    //      For each node having this one as dependency
    //      Decrement the count of incoming edges for the dependent node
    //      If count is 0, it has no incoming edges anymore, push it to the queue
    
    let mut sorted = Vec::new();
    while let Some(edge) = no_incoming_edges.pop_back() {
        sorted.push(edge);
        incoming_edges_count.remove(&edge);
        for adjancent in graph.get_adjacent_vertices(edge).unwrap_or(&vec![]) {
           if let Some(count) = incoming_edges_count.get_mut(&adjancent.0) {
               *count -= 1;
                if *count == 0 {
                    incoming_edges_count.remove(&adjancent.0);
                    no_incoming_edges.push_front(adjancent.0);
                }
            }
        }
    }
    
    if incoming_edges_count.is_empty() {
        Some(sorted)
    } else {
        None //we have cycles
    }    
}

#[cfg(test)]
mod test_search {
    use crate::topological_sort;

    #[test]
    fn test_sort_with_cycle() {
        let mut graph = super::Graph::new();
        graph.add_vertex(1);
        graph.add_vertex(2);
        graph.add_vertex(3);
        graph.add_edge(1, 2, 0);
        graph.add_edge(2, 3, 0);
        graph.add_edge(3, 1, 0);
        let sort = topological_sort(&graph);
        assert_eq!(true, sort.is_none());
    }

    #[test]
    fn test_sort_one_variant() {
        let mut graph = super::Graph::new();
        graph.add_vertex(1);
        graph.add_vertex(2);
        graph.add_vertex(3);
        graph.add_vertex(4);
        graph.add_vertex(5);
        graph.add_vertex(6);
        graph.add_vertex(7);
        graph.add_edge(1, 2, 0);
        graph.add_edge(1, 3, 0);
        graph.add_edge(2, 4, 0);
        graph.add_edge(2, 5, 0);
        graph.add_edge(3, 6, 0);
        graph.add_edge(3, 7, 0);
        let sort = topological_sort(&graph);
        let sort = sort.unwrap();
        assert_eq!(Some(vec![1, 2, 3, 4, 5, 6, 7]), Some(sort));
    }

    #[test]
    fn test_sort_many_variant() {

        let mut graph1 = super::Graph::new();
        graph1.add_vertex(2);        
        graph1.add_vertex(3);        
        graph1.add_vertex(5);
        graph1.add_vertex(7);        
        graph1.add_vertex(8);
        graph1.add_vertex(9);        
        graph1.add_vertex(10);
        graph1.add_vertex(11);        
        
        graph1.add_edge(5, 11, 0);
        graph1.add_edge(7, 11, 0);
        graph1.add_edge(7, 8, 0);
        graph1.add_edge(3, 8, 0);
        graph1.add_edge(3, 10, 0);
        graph1.add_edge(11, 2, 0);
        graph1.add_edge(11, 9, 0);
        graph1.add_edge(11, 10, 0);
        graph1.add_edge(8, 9, 0);
        topological_sort(&graph1);        

    }
}