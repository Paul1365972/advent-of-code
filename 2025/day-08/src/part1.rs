use std::collections::VecDeque;
use itertools::Itertools;
use petgraph::{graph::UnGraph, unionfind::UnionFind, visit::{EdgeRef, NodeIndexable}};

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

    let connections = 1000;
    let mut graph = Graph::with_capacity(boxes.len(), connections);
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

    for _ in 0..connections {
        let (i, j) = distances.pop_front().unwrap();
        graph.add_edge(i, j, ());
    }

    let mut node_sets = UnionFind::new(graph.node_bound());
    for edge in graph.edge_references() {
        let (a, b) = (edge.source(), edge.target());
        node_sets.union(graph.to_index(a), graph.to_index(b));
    }

    let counts = node_sets.into_labeling().into_iter().counts();
    let result = counts.into_values().sorted().rev().take(3).product::<usize>();

    println!("Result: {}", result);
}
