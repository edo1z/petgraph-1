use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::fs::File;
use std::io::prelude::*;

fn create_graph(nodes_len: usize, edges: &Vec<(usize, usize)>) -> Graph<i32, i32> {
    let mut g = Graph::<i32, i32>::new();
    for i in 0..nodes_len {
        g.add_node(i as i32);
    }
    for edge in edges.iter() {
        g.add_edge(NodeIndex::new(edge.0), NodeIndex::new(edge.1), 1);
    }
    g
}

fn main() -> std::io::Result<()> {
    let edges: Vec<(usize, usize)> = vec![(0, 1), (0, 2), (1, 2), (1, 4), (2, 3)];
    let g = create_graph(5, &edges);
    let dot = Dot::with_config(&g, &[Config::EdgeNoLabel]);
    let mut file = File::create("out.dot")?;
    file.write_all(dot.to_string().as_bytes())?;
    Ok(())
}
