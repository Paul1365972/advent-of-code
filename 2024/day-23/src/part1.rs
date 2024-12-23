use itertools::{iproduct, Itertools};
use petgraph::{graph::NodeIndex, prelude::StableUnGraph};
use rustc_hash::{FxHashMap, FxHashSet};

fn main() {
    let input = include_str!("../input.txt");

    let connections = input
        .lines()
        .map(|line| (&line[0..2], &line[3..5]))
        .collect_vec();

    let mut idx_map = FxHashMap::default();
    let mut graph = StableUnGraph::default();
    for name in connections.iter().flat_map(|&(a, b)| [a, b]) {
        let idx: NodeIndex = graph.add_node(name);
        idx_map.insert(name, idx);
    }
    for (a, b) in connections.iter() {
        graph.add_edge(idx_map[a], idx_map[b], ());
    }

    let mut triangles = FxHashSet::default();
    for node in graph.node_indices() {
        if graph[node].starts_with("t") {
            for (a, b) in iproduct!(graph.neighbors(node), graph.neighbors(node)) {
                if a == b {
                    continue;
                }
                if graph.contains_edge(a, b) {
                    let mut triangle = [graph[node], graph[a], graph[b]];
                    triangle.sort();
                    triangles.insert(triangle);
                }
            }
        }
    }

    println!("Result: {}", triangles.len());
}
