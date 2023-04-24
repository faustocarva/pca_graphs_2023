use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Graph<V: Hash + Eq + Clone, E: Hash + Eq + Clone> {
    adj_list: HashMap<V, Vec<(V, E)>>,
}

impl<V: Hash + Eq + Clone, E: Hash + Eq + Clone> Graph<V, E> {
    pub fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: V) {
        self.adj_list.entry(vertex).or_insert(Vec::new());
    }

    pub fn add_edge(&mut self, from: V, to: V, value: E) {
        self.adj_list.entry(from).or_insert(Vec::new()).push((to, value));
    }

    // pub fn add_edge(&mut self, from: V, to: V) {
    //     self.adj_list.entry(from).or_insert(Vec::new()).push((to,0u32));
    // }

    pub fn get_adjacent_vertices(&self, v: V) -> Option<&Vec<(V, E)>> {
        self.adj_list.get(&v)
    }

    pub fn adj_list(&self) -> &HashMap<V, Vec<(V, E)>> {
        &self.adj_list
    }

}


#[cfg(test)]
mod test_graph {
    use super::Graph;
    #[test]
    fn test_add_vertex() {
        let mut g: Graph<u32, u32> = Graph::new();
        g.add_vertex(1);        
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
    fn test_get_vertices() {
    }

    #[test]
    fn test_get_adj_list() {
    }

}
