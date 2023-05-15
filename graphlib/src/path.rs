use super::{EdgeType, Graph, GraphEdgeTrait, GraphVertexTrait};

/// Shortest path algorithms.

pub fn dijkstra<V: GraphVertexTrait, E: GraphEdgeTrait, T: EdgeType>(
    graph: &Graph<V, E, T>,
    start: V,
) -> Option<(E, Vec<(V, V)>)> {
    None
}