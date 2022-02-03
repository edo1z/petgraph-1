use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::prelude::Dfs;
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

fn create_dotfile(g: &Graph<i32, i32>, file_name: &str) -> std::io::Result<()> {
    let dot = Dot::with_config(&g, &[Config::EdgeNoLabel]);
    let mut file = File::create(file_name)?;
    file.write_all(dot.to_string().as_bytes())?;
    Ok(())
}

fn dfs(g: &Graph<i32, i32>) -> Graph<i32, i32> {
    let start: usize = 0;
    let mut dfs = Dfs::new(&g, NodeIndex::new(start));
    let mut route = vec![];
    let mut prev = start;
    let mut max_idx = 0;
    while let Some(node_idx) = dfs.next(&g) {
        let idx = node_idx.index();
        if idx != start {
            if idx > max_idx {
                max_idx = idx
            }
            route.push((prev, idx));
            prev = node_idx.index();
        }
    }
    create_graph(max_idx + 1, &route)
}

fn main() -> std::io::Result<()> {
    let edges: Vec<(usize, usize)> = vec![
        (0, 1),
        (0, 2),
        (1, 2),
        (1, 4),
        (2, 3),
        (2, 7),
        (0, 5),
        (1, 6),
    ];
    let g = create_graph(8, &edges);
    create_dotfile(&g, "graph.dot")?;
    let g2 = dfs(&g);
    create_dotfile(&g2, "dfs.dot")?;
    Ok(())
}
