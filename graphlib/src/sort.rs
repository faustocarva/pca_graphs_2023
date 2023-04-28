use super::{Graph, GraphElemTrait};


/// Performs topological sort using the Kahn's algorithm.
/// Returns a Vec storing the vertices in a the topological order.
pub fn topological_sort<V: GraphElemTrait, E: GraphElemTrait>(graph: &Graph<V, E>) -> Option<Vec<V>> {
    // 1) Preparation:
    //  Build a map of edges, organised from source to destinations
    //  Also, count the number of incoming edges by node

    // 2) Now Kahn's algorithm:
    // Add nodes that have no incoming edges to a queue


    // 3) For each node in this "O-incoming-edge-queue"

    // 4) Return the order or error (if cycles were detected)
    None
}

#[cfg(test)]
mod test_search {

    #[test]
    fn test_sort_none() {}
}