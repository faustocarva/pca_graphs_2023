use super::{Graph, GraphElemTrait, Undirected};
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;

/// Source for Minimal Spanning Tree algorithms.
/// First: Kruskal
/// Second: Prim
///
/// Kruskal (UNION/FIND DISJOINT_SET)
///     Sort edges by ascending edge weight
///     Loop over sorted edges
///     Take two vertices
///     If vertices are UNIFIED (FIND in DISJOINT_SET), don't inclued vertices
///     Else, UNION on those two edges
///     Terminate when all edges have been processed, or all vertices have been UNIFIED

#[allow(unused_variables)]
pub fn kruskal<V: GraphElemTrait, E: GraphElemTrait>(
    graph: &Graph<V, E, Undirected>,
) -> Option<(E, Vec<V>)> {
    None
}
