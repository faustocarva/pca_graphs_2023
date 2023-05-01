use std::collections::{HashSet,HashMap};
use std::fmt::Debug;
use std::hash::Hash;

///TODO: 
/// * Make two types of graphs: Directed and Undirected
/// 
pub trait GraphElemTrait: Hash + Eq + Clone + Copy {}
impl<T> GraphElemTrait for T where T: Hash + Eq + Clone + Copy {}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Graph<V: GraphElemTrait, E: GraphElemTrait> {
    adj_list: HashMap<V, Vec<(V, E)>>,
}

impl<V: GraphElemTrait, E: GraphElemTrait> Graph<V, E> {
    pub fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: V) {
        self.adj_list.entry(vertex).or_insert(Vec::new());
    }

    pub fn add_edge(&mut self, from: V, to: V, value: E) {
        if self.adj_list.get(&to).is_some() {
            if let Some(neighbours) = self.adj_list.get_mut(&from) {
                if !neighbours.contains(&(to.clone(), value.clone())) {
                    neighbours.push((to, value));
                }
            }
        }
    }

    // There is no method/function overload in rust, and there is not default parameter
    // pub fn add_edge(&mut self, from: V, to: V) {
    //     self.adj_list.entry(from).or_insert(Vec::new()).push((to,0u32));
    // }

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
        let mut flat_graph: Vec<(V,V)> = Vec::new();
        for from in &self.adj_list {
            for to in from.1 {
                flat_graph.push((*from.0, to.0));
            }
        }
        flat_graph
    }    


}

#[cfg(test)]
mod test_graph {
    use super::Graph;
    #[test]
    fn test_add_vertex() {
        let mut g: Graph<u32, u32> = Graph::new();
        g.add_vertex(1);
        g.add_vertex(2);        
        g.add_edge(1, 2, 0);
        g.add_edge(1, 2, 0);        
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
        assert_eq!(
            g.get_adjacent_vertices(2).unwrap(),
            &vec![(3, 0)]
        );        
    }
    #[test]
    fn test_add_edges_strings_cities() {
        let mut g  = Graph::new();
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
        let mut g  = Graph::new();
        g.add_vertex("NYC");
        g.add_vertex("MTL");
        g.add_vertex("TOR"); 
        g.add_edge("NYC", "MTL", 530);        
        println!("{:?}", g.vertices());
        assert_eq!(
            g.vertices(),
            [("NYC"), ("MTL"), ("TOR")]
                .iter()
                .collect()
        );

    }

    #[test]
    fn test_contains_and_count() {
        let mut g  = Graph::new();
        g.add_vertex("NYC");
        g.add_vertex("MTL");
        g.add_vertex("TOR"); 
        assert_eq!(
            g.vertices_count(), 3
        );
        g.add_edge("NYC", "MTL", 530);
        assert_eq!(
            g.contains(&String::from("NYC")), true
        );

    }
}
