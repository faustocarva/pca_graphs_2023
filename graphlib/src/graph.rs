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