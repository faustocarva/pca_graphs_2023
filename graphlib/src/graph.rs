use std::cmp::PartialOrd;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;

pub trait GraphEdgeTrait:
    Default + Hash + Eq + Clone + Copy + PartialOrd + Add + Div + AddAssign + Ord
{
}
impl<T> GraphEdgeTrait for T where
    T: Default + Hash + Eq + Clone + Copy + PartialOrd + Add + Div + AddAssign + Ord
{
}

pub trait GraphVertexTrait: Hash + Eq + Clone + Copy + PartialOrd {}
impl<T> GraphVertexTrait for T where T: Hash + Eq + Clone + Copy + PartialOrd {}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Graph<V: GraphVertexTrait, E: GraphEdgeTrait, T = Directed> {
    adj_list: HashMap<V, Vec<(V, E)>>,
    phantom: PhantomData<T>, //Hackish variable to make rustc keep quiet about T
}

#[derive(Debug)]
pub enum Directed {}

#[derive(Debug)]
pub enum Undirected {}

pub trait EdgeType {
    fn is_directed() -> bool;
}

impl EdgeType for Directed {
    fn is_directed() -> bool {
        true
    }
}

impl EdgeType for Undirected {
    fn is_directed() -> bool {
        false
    }
}

impl<V, E> Graph<V, E, Directed>
where
    V: GraphVertexTrait,
    E: GraphEdgeTrait,
{
    pub fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
            phantom: PhantomData,
        }
    }
}

impl<V, E> Graph<V, E, Undirected>
where
    V: GraphVertexTrait,
    E: GraphEdgeTrait,
{
    pub fn new_undirected() -> Self {
        Graph {
            adj_list: HashMap::new(),
            phantom: PhantomData,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct EdgeComparator<V: GraphVertexTrait, E: GraphEdgeTrait>(pub V, pub V, pub E);

impl<V, E> PartialOrd for EdgeComparator<V, E>
where
    V: GraphVertexTrait,
    E: GraphEdgeTrait,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<V, E> Ord for EdgeComparator<V, E>
where
    V: GraphVertexTrait,
    E: GraphEdgeTrait,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let left = other.2;
        let right = self.2;

        right.cmp(&left)
    }
}

impl<V, E, T> Graph<V, E, T>
where
    V: GraphVertexTrait,
    E: GraphEdgeTrait,
    T: EdgeType,
{
    pub fn add_vertex(&mut self, vertex: V) {
        self.adj_list.entry(vertex).or_insert(Vec::new());
    }

    // There is no method/function overload in rust, and there is not default parameter
    pub fn add_edge(&mut self, from: V, to: V, value: E) {
        self.adj_list.entry(to).or_insert(Vec::new());
        self.adj_list.entry(from).or_insert(Vec::new());

        if self.adj_list.get(&to).is_some() {
            if let Some(neighbours) = self.adj_list.get_mut(&from) {
                if !neighbours.contains(&(to, value)) {
                    neighbours.push((to, value));
                }
            }
        }
        if !T::is_directed() && self.adj_list.get(&from).is_some() {
            if let Some(neighbours) = self.adj_list.get_mut(&to) {
                if !neighbours.contains(&(from, value)) {
                    neighbours.push((from, value));
                }
            }
        }
    }

    pub fn get_adjacent_vertices(&self, v: V) -> Option<&Vec<(V, E)>> {
        self.adj_list.get(&v)
    }

    pub fn adj_list(&self) -> &HashMap<V, Vec<(V, E)>> {
        &self.adj_list
    }

    pub fn vertices_count(&self) -> usize {
        self.adj_list.len()
    }

    pub fn contains(&self, vertex: V) -> bool {
        self.adj_list().get(&vertex).is_some()
    }

    pub fn vertices(&self) -> HashSet<&V> {
        self.adj_list().keys().collect()
    }

    pub fn edges(&self) -> Vec<(V, V)> {
        let mut flat_graph: Vec<(V, V)> = Vec::new();
        for from in &self.adj_list {
            for to in from.1 {
                flat_graph.push((*from.0, to.0));
            }
        }
        flat_graph
    }

    pub fn edges_with_weights(&self, order: std::cmp::Ordering) -> Vec<(V, V, E)> {
        let mut flat_graph: Vec<(V, V, E)> = Vec::new();
        for from in &self.adj_list {
            for to in from.1 {
                flat_graph.push((*from.0, to.0, to.1));
            }
        }

        flat_graph.sort_unstable_by(|a, b| {
            let weight_a = a.2;
            let weight_b = b.2;
            match order {
                std::cmp::Ordering::Less => weight_a
                    .partial_cmp(&weight_b)
                    .unwrap_or(std::cmp::Ordering::Less),
                std::cmp::Ordering::Greater => weight_b
                    .partial_cmp(&weight_a)
                    .unwrap_or(std::cmp::Ordering::Less),
                _ => weight_a
                    .partial_cmp(&weight_b)
                    .unwrap_or(std::cmp::Ordering::Less),
            }
        });
        flat_graph
    }
}

#[cfg(test)]
mod test_graph {
    use super::Graph;
    use std::cmp::Ordering;
    #[test]
    fn test_add_edges_duplicated_directed_graph() {
        let mut g = Graph::new();
        g.add_vertex(1);
        g.add_vertex(2);
        assert_eq!(g.edges().len(), 0);
        g.add_edge(1, 2, 0);
        g.add_edge(1, 2, 0);
        assert_eq!(g.edges().len(), 1);
        println!("{:?}", g);
    }

    #[test]
    fn test_add_edges_undirected_graph() {
        let mut g = Graph::new_undirected();
        g.add_vertex(1);
        g.add_vertex(2);
        assert_eq!(g.edges().len(), 0);
        g.add_edge(1, 2, 0);
        assert_eq!(g.edges().len(), 2);
        g.add_edge(1, 2, 0);
        assert_eq!(g.edges().len(), 2);
        println!("{:?}", g);
    }

    #[test]
    fn test_add_edges() {
        let mut g: Graph<u32, u32> = Graph::new();
        g.add_vertex(1);
        g.add_vertex(2);
        g.add_vertex(3);
        g.add_edge(1, 2, 0);
        g.add_edge(2, 3, 0);
        println!("{:?}", g);
        assert_eq!(g.get_adjacent_vertices(2).unwrap(), &vec![(3, 0)]);
    }
    #[test]
    fn test_add_edges_strings_cities() {
        let mut g = Graph::new();
        g.add_vertex("NYC");
        g.add_vertex("MTL");
        g.add_vertex("TOR");
        g.add_edge("NYC", "MTL", 530);
        g.add_edge("NYC", "TOR", 560);
        g.add_edge("MTL", "TOR", 525);
        println!("{:?}", g);
        assert_eq!(
            g.get_adjacent_vertices("NYC").unwrap(),
            &vec![("MTL", 530), ("TOR", 560)]
        );
    }

    #[test]
    fn test_get_vertices() {
        let mut g = Graph::new();
        g.add_vertex("NYC");
        g.add_vertex("MTL");
        g.add_vertex("TOR");
        g.add_edge("NYC", "MTL", 530);
        println!("{:?}", g.vertices());
        assert_eq!(g.vertices(), [("NYC"), ("MTL"), ("TOR")].iter().collect());
    }

    #[test]
    fn test_contains_x_edges() {
        let mut g = Graph::new();
        g.add_vertex("NYC");
        g.add_vertex("MTL");
        g.add_vertex("TOR");
        assert_eq!(g.edges().len(), 0);
        g.add_edge("NYC", "MTL", 530);
        assert_eq!(g.edges().len(), 1);
    }

    #[test]
    fn test_contains_and_count() {
        let mut g = Graph::new();
        g.add_vertex("NYC");
        g.add_vertex("MTL");
        g.add_vertex("TOR");
        assert_eq!(g.vertices_count(), 3);
        g.add_edge("NYC", "MTL", 530);
        assert_eq!(g.contains(&String::from("NYC")), true);
    }

    #[test]
    fn test_add_edges_with_weights_and_order() {
        let mut g = Graph::new();
        g.add_vertex("NYC");
        g.add_vertex("MTL");
        g.add_vertex("TOR");
        assert_eq!(g.vertices_count(), 3);
        g.add_edge("NYC", "MTL", 530);
        g.add_edge("TOR", "NYC", 2);
        g.add_edge("MTL", "TOR", 590);
        let greater = g.edges_with_weights(Ordering::Greater);
        //println!("{:?}", ordered);
        assert_eq!(
            greater,
            vec![("MTL", "TOR", 590), ("NYC", "MTL", 530), ("TOR", "NYC", 2)]
        );
        let less = g.edges_with_weights(Ordering::Less);
        assert_eq!(
            less,
            vec![("TOR", "NYC", 2), ("NYC", "MTL", 530), ("MTL", "TOR", 590)]
        );
    }
}
