use std::collections::VecDeque;
use itertools::Itertools;
use petgraph::graph::UnGraph;

pub type Graph = UnGraph<(i64, i64, i64), (), u32>;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();

    let boxes = input
        .trim()
        .lines()
        .map(|line| {
            line.split(",")
                .map(|s| s.parse().unwrap())
                .collect_tuple::<(i64, i64, i64)>()
                .unwrap()
        })
        .collect_vec();

    let mut graph = Graph::with_capacity(boxes.len(), 0);
    for item in boxes {
        graph.add_node(item);
    }
    let mut distances: VecDeque<_> = graph
        .node_indices()
        .cartesian_product(graph.node_indices())
        .filter(|(i, j)| i < j)
        .sorted_by_cached_key(|(i, j)| {
            let a = graph[*i];
            let b = graph[*j];
            (a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1) + (a.2 - b.2) * (a.2 - b.2)
        })
        .collect();

    let mut result = 0;
    loop {
        let (i, j) = distances.pop_front().unwrap();
        graph.add_edge(i, j, ());
        if petgraph::algo::connected_components(&graph) == 1 {
            result = graph[i].0 * graph[j].0;
            break;
        }
    }

    println!("Result: {}", result);
}
