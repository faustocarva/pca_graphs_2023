use std::collections::HashMap;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Graph {
    adj_list: HashMap<u32, Vec<u32>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, v: u32) {
        self.adj_list.entry(v).or_insert(Vec::new());
    }

    pub fn add_edge(&mut self, u: u32, v: u32) {
        self.adj_list.entry(u).or_insert(Vec::new()).push(v);
    }

    pub fn get_adjacent_vertices(&self, v: u32) -> Option<&Vec<u32>> {
        self.adj_list.get(&v)
    }
}


#[cfg(test)]
mod test_graph {
    use super::Graph;
    #[test]
    fn test_add_vertex() {
        let mut g = Graph::new();
        g.add_vertex(1);        
    }

    #[test]
    fn test_add_edges() {
        let mut g = Graph::new();
        g.add_vertex(1);
        g.add_vertex(2);
        g.add_vertex(3);        
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        println!("{:?}", g);
        assert_eq!(
            g.get_adjacent_vertices(2).unwrap(),
            &vec![3]
        );        
    }
}
