use graphlib::*;

fn main() {
    let mut g = Graph::new();
    g.add_vertex(1);
    g.add_vertex(2);
    g.add_vertex(3);
    g.add_edge(1, 2);
    g.add_edge(2, 3);
    g.add_edge(3, 1);
    println!("{:?}", g);
}
