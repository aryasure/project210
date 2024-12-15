use petgraph::graph::{UnGraph, NodeIndex};
use petgraph::algo::dijkstra;
use std::collections::HashMap;

/// calculate closeness centrality for each node
pub fn calculate_closeness_centrality(graph: &UnGraph<u32, ()>) -> HashMap<NodeIndex, f64> {
    let mut centrality = HashMap::new();

    for node in graph.node_indices() {
        let shortest_paths = dijkstra(&graph, node, None, |_| 1);
        let total_distance: usize = shortest_paths.values().sum();
        let num_nodes = graph.node_count();

        if total_distance > 0 {
            let closeness = (num_nodes - 1) as f64 / total_distance as f64;
            centrality.insert(node, closeness);
        }
    }

    display_top_bottom_closeness(&centrality, 5);

    centrality
}

/// display top and bottom nodes based on closeness centrality
pub fn display_top_bottom_closeness(
    centrality: &HashMap<NodeIndex, f64>,
    top_n: usize,
) {
    let mut centrality_vec: Vec<(&NodeIndex, &f64)> = centrality.iter().collect();
    centrality_vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));

    println!("\ntop {} closeness centrality scores:", top_n);
    for (node, score) in centrality_vec.iter().take(top_n) {
        println!("node {:?}: {:.4}", node, score);
    }

    println!("\nbottom {} closeness centrality scores:", top_n);
    for (node, score) in centrality_vec.iter().rev().take(top_n) {
        println!("node {:?}: {:.4}", node, score);
    }
}
