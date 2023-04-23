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
            break;
        }        
        if let Some(neighbors) = graph.adjacency_list().get(&node) {
            for neighbor in neighbors {
                queue.push_back(*neighbor);
            }
        }
    }
    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

#[cfg(test)]
mod test_bfs {
    #[test]
    fn test_bfs_find() {
    }

    #[test]
    fn test_bfs_donot_find() {
    }
}


