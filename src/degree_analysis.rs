use petgraph::graph::UnGraph;
use std::collections::HashMap;

/// calculate and return the degree distribution of the graph
pub fn calculate_degree_distribution(graph: &UnGraph<u32, ()>) -> Vec<(usize, usize)> {
    let mut degree_distribution: Vec<(usize, usize)> = graph
        .node_indices()
        .map(|node| (graph.neighbors(node).count(), node.index()))
        .collect();

    degree_distribution.sort_by(|a, b| b.0.cmp(&a.0));
    degree_distribution
}

/// display top and bottom nodes based on their degree
pub fn display_top_bottom(degree_distribution: &Vec<(usize, usize)>, top_n: usize) {
    println!("\ntop {} nodes by degree:", top_n);
    println!("{:<10} {:<10}", "degree", "node");
    println!("----------------------");
    for (degree, node) in degree_distribution.iter().take(top_n) {
        println!("{:<10} {:<10}", degree, node);
    }

    println!("\nbottom {} nodes by degree:", top_n);
    println!("{:<10} {:<10}", "degree", "node");
    println!("----------------------");
    for (degree, node) in degree_distribution.iter().rev().take(top_n) {
        println!("{:<10} {:<10}", degree, node);
    }
}

/// display a histogram of degree counts
pub fn display_degree_histogram(degree_distribution: &Vec<(usize, usize)>) {
    let mut histogram = HashMap::new();

    for (degree, _) in degree_distribution {
        *histogram.entry(*degree).or_insert(0) += 1;
    }

    println!("\ndegree histogram (top 10):");
    println!("{:<10} {:<10}", "degree", "count");
    println!("----------------------");

    let mut sorted_histogram: Vec<(&usize, &usize)> = histogram.iter().collect();
    sorted_histogram.sort_by(|a, b| b.0.cmp(a.0));

    for (degree, count) in sorted_histogram.iter().take(10) {
        println!("{:<10} {:<10}", degree, count);
    }
}

/// calculate and display basic statistics for the degree distribution
pub fn calculate_statistics(degree_distribution: &Vec<(usize, usize)>) {
    let degrees: Vec<usize> = degree_distribution.iter().map(|(deg, _)| *deg).collect();

    let sum: usize = degrees.iter().sum();
    let avg = sum as f64 / degrees.len() as f64;

    let min_degree = degrees.iter().min().unwrap_or(&0);
    let max_degree = degrees.iter().max().unwrap_or(&0);

    println!("\ndegree statistics:");
    println!("----------------------");
    println!("average degree: {:.2}", avg);
    println!("minimum degree: {}", min_degree);
    println!("maximum degree: {}", max_degree);
}
