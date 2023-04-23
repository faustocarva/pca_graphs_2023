use std::collections::{VecDeque,HashSet};
use super::Graph;

pub fn breadth_first_search(graph: &Graph, start: u32, target: u32) -> Option<Vec<u32>> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut result = Vec::new();
    queue.push_back(start);
    while let Some(node) = queue.pop_front() {
        if visited.contains(&node) {
            continue;
        }        
        visited.insert(node);
        result.push(node);

        if node == target {
            return Some(result);
        }        
        if let Some(neighbors) = graph.adj_list().get(&node) {
            for neighbor in neighbors {
                queue.push_back(*neighbor);
            }
        }
    }
    None
}

// pub fn depth_first_search(graph: &Graph, start: u32, target: u32) -> Option<Vec<u32>> {
//     None
// }


#[cfg(test)]
mod test_bfs {
    #[test]
    fn test_bfs_find() {
        let mut graph = super::Graph::new();
        graph.add_vertex(1);
        graph.add_vertex(2);
        graph.add_vertex(3);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        assert_eq!(
            super::breadth_first_search(&graph, 1, 3).is_none(),
            false
        );

        let result = super::breadth_first_search(&graph, 1, 3);
        println!("{:?}", result);
    }

    #[test]
    fn test_bfs_find_none() {
        let mut graph = super::Graph::new();
        graph.add_vertex(1);
        graph.add_vertex(2);
        graph.add_vertex(3);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        assert_eq!(
            super::breadth_first_search(&graph, 1, 4).is_none(),
            true
        );
    }
}


