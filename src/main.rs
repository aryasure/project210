mod graph_loader;
mod degree_analysis;
mod centrality;

fn main() {
    env_logger::init();

    let filename = "facebook_combined.txt";

    match graph_loader::load_graph(filename) {
        Ok(graph) => {
            println!("graph loaded with {} nodes and {} edges", graph.node_count(), graph.edge_count());
            
            let degree_distribution = degree_analysis::calculate_degree_distribution(&graph);
          
	    let top_n = 10;
            
	    degree_analysis::display_top_bottom(&degree_distribution, top_n);
            degree_analysis::display_degree_histogram(&degree_distribution);
            degree_analysis::calculate_statistics(&degree_distribution);

            let _centrality_scores = centrality::calculate_closeness_centrality(&graph);
        }
        Err(e) => eprintln!("error loading graph: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::UnGraph;

    #[test]
    fn test_graph_loading() {
        let filename = "test_graph.txt";

        // creat test graph file
        std::fs::write(filename, "1 2\n2 3\n").expect("Failed to create test graph file");

        let graph = graph_loader::load_graph(filename).unwrap();
        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 2);

        // clean up the test file
        std::fs::remove_file(filename).expect("failed to delete test graph file");
    }

    #[test]
    fn test_degree_distribution() {
        let mut graph = UnGraph::<u32, ()>::new_undirected();
        let node1 = graph.add_node(1);
        let node2 = graph.add_node(2);
        let node3 = graph.add_node(3);
        graph.add_edge(node1, node2, ());
        graph.add_edge(node2, node3, ());

        let mut degree_count = std::collections::HashMap::new();
        for node in graph.node_indices() {
            let degree = graph.neighbors(node).count();
            *degree_count.entry(degree).or_insert(0) += 1;
        }

        assert_eq!(degree_count.get(&1), Some(&2));
        assert_eq!(degree_count.get(&2), Some(&1));
    }

    #[test]
    fn test_empty_graph() {
        let graph = UnGraph::<u32, ()>::new_undirected();
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }
}

