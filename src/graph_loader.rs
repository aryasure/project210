use petgraph::graph::UnGraph;
use std::fs::File;
use std::io::{self, BufRead};

pub fn load_graph(filename: &str) -> io::Result<UnGraph<u32, ()>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut graph = UnGraph::<u32, ()>::new_undirected();
    let mut node_map = std::collections::HashMap::new();
    let mut next_index = 0;

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            continue;
        }

        let node1: u32 = parts[0].parse().unwrap();
        let node2: u32 = parts[1].parse().unwrap();

        let index1 = *node_map.entry(node1).or_insert_with(|| {
            let idx = next_index;
            next_index += 1;
            graph.add_node(node1);
            idx
        });

        let index2 = *node_map.entry(node2).or_insert_with(|| {
            let idx = next_index;
            next_index += 1;
            graph.add_node(node2);
            idx
        });

        graph.add_edge(index1.into(), index2.into(), ());
    }

    Ok(graph)
}

