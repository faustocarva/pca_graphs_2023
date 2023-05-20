use super::{EdgeType, Graph, GraphEdgeTrait, GraphVertexTrait};
use std::collections::BTreeMap;

/// All-Pairs Shortest Path algorithms.

///  Floyd-Warshall algorithm

#[allow(unused_variables)]
#[allow(dead_code)]
pub fn floyd_warshall<V: GraphVertexTrait, E: GraphEdgeTrait, T: EdgeType>(
    graph: &Graph<V, E, T>,
) -> Option<BTreeMap<V, BTreeMap<V, E>>> {
    None
}
