use std::collections::{VecDeque,HashSet};
use std::hash::Hash;
use super::Graph;

/// Performs the Breadth First Search algorithm on the input graph
/// Returns a Vec storing the vertices the were taken
pub fn breadth_first_search<V: Hash + Eq + Clone, E: Hash + Eq + Clone>(graph: &Graph<V, E>, start: V, target: V) -> Option<Vec<V>> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut result = Vec::new();
    queue.push_back(start);
    while let Some(node) = queue.pop_front() {
        if visited.contains(&node) {
            continue;
        }        
        visited.insert(node.clone());
        result.push(node.clone());

        if node == target {
            return Some(result);
        }        
        if let Some(neighbors) = graph.adj_list().get(&node) {
            for neighbor in neighbors {
                queue.push_back(neighbor.0.clone());
            }
        }
    }
    None
}

/// Performs the Depth First Search algorithm on the input graph
/// Returns a Vec storing the vertices the were taken
pub fn depth_first_search<V: Hash + Eq + Clone, E: Hash + Eq + Clone>(graph: &Graph<V, E>, start: V, target: V) -> Option<Vec<V>> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut result = Vec::new();
    queue.push_back(start);
    while let Some(node) = queue.pop_front() {
        result.push(node.clone());

        if node == target {
            return Some(result);
        }        
        if let Some(neighbors) = graph.adj_list().get(&node) {
            // Reverse the order, so we can still use VecDeque
            for neighbor in neighbors.iter().rev() {            
                if visited.insert(neighbor.0.clone()) {
                    queue.push_front(neighbor.0.clone());
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod test_search {

    #[test]
    fn test_bfs_find() {
        {
            let mut graph = super::Graph::new();
            graph.add_vertex(1);
            graph.add_vertex(2);
            graph.add_vertex(3);
            graph.add_edge(1, 2, 0);
            graph.add_edge(2, 3, 0);
            assert_eq!(
                super::breadth_first_search(&graph, 1, 3).is_none(),
                false
            );
    
            let expected_path = vec![1, 2, 3];
            assert_eq!(
                super::breadth_first_search(&graph, 1, 3),
                Some(expected_path)
            ); 
        }
        {
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

            println!("{:?}", graph);                    
            
            let result = super::breadth_first_search(&graph, 1, 7);
            let expected_path = vec![1, 2, 3, 4, 5, 6, 7];
            assert_eq!(result, Some(expected_path));
        }
        {
            let mut g  = super::Graph::new();
            g.add_vertex("NYC");
            g.add_vertex("MTL");
            g.add_vertex("TOR");        
            g.add_edge("NYC", "MTL", 530);
            g.add_edge("NYC", "TOR", 560);
            g.add_edge("MTL", "TOR", 525);
            let result = super::breadth_first_search(&g, "NYC", "TOR");
            let expected_path = vec!["NYC", "MTL", "TOR"];
            assert_eq!(result, Some(expected_path));
    
        }

    }
    
    #[test]
    fn test_bfs_find_none() {        
        let mut graph = super::Graph::new();
        graph.add_vertex(1);
        graph.add_vertex(2);
        graph.add_vertex(3);
        graph.add_edge(1, 2, 0);
        graph.add_edge(2, 3, 0);
        assert_eq!(
            super::breadth_first_search(&graph, 1, 4).is_none(),
            true
        );
    }

    #[test]
    fn test_dfs_find_none() {
        let mut graph = super::Graph::new();
        graph.add_vertex(1);
        graph.add_vertex(2);
        graph.add_vertex(3);
        graph.add_edge(1, 2, 0);
        graph.add_edge(2, 3, 0);
        assert_eq!(
            super::depth_first_search(&graph, 1, 4).is_none(),
            true
        );

    }

    #[test]        
    fn test_dfs_find() {
            let mut graph1 = super::Graph::new();
            graph1.add_vertex(1);
            graph1.add_vertex(2);
            graph1.add_vertex(3);
            graph1.add_edge(1, 2, 0);
            graph1.add_edge(2, 3, 0);
            assert_eq!(
                super::depth_first_search(&graph1, 1, 4).is_none(),
                true
            );    
            let mut graph2 = super::Graph::new();
            graph2.add_vertex(1);
            graph2.add_vertex(2);
            graph2.add_vertex(3);
            graph2.add_vertex(4);
            graph2.add_vertex(5);
            graph2.add_vertex(6);
            graph2.add_vertex(7);
            graph2.add_edge(1, 2, 0);
            graph2.add_edge(1, 3, 0);
            graph2.add_edge(2, 4, 0);
            graph2.add_edge(2, 5, 0);
            graph2.add_edge(3, 6, 0);
            graph2.add_edge(3, 7, 0);

            println!("{:?}", graph2);
            
            let result = super::depth_first_search(&graph2, 1, 7);
            let expected_path = vec![1, 2, 4, 5, 3, 6, 7];
            assert_eq!(result, Some(expected_path));
    }

}


