use graphlib::*;

fn main() {
    let mut graph = Graph::new();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_vertex(3);
    graph.add_vertex(4);
    graph.add_vertex(5);
    graph.add_vertex(6);
    graph.add_vertex(7);
    graph.add_edge(1, 2, 0);
    graph.add_edge(1, 3, 0);
    graph.add_edge(2, 4, 0);
    graph.add_edge(2, 5, 0 );
    graph.add_edge(3, 6, 0);
    graph.add_edge(3, 7, 0 );
    println!("{:?}", graph);
}
