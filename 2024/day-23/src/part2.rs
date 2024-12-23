use itertools::Itertools;
use petgraph::{graph::NodeIndex, prelude::StableUnGraph};
use rustc_hash::FxHashMap;

fn main() {
    let input = include_str!("../input.txt");

    let connections = input
        .lines()
        .map(|line| (&line[0..2], &line[3..5]))
        .collect_vec();

    let mut idx_map = FxHashMap::default();
    let mut graph = StableUnGraph::default();
    for name in connections.iter().flat_map(|&(a, b)| [a, b]) {
        let idx = graph.add_node(name);
        idx_map.insert(name, idx);
    }
    for (a, b) in connections.iter() {
        graph.add_edge(idx_map[a], idx_map[b], ());
    }

    let mut groups: Vec<Vec<NodeIndex>> = Vec::new();
    let mut to_add = Vec::new();
    for node in graph.node_indices() {
        to_add.push(vec![node]);
        for group in groups.iter() {
            if group.iter().all(|other| graph.contains_edge(node, *other)) {
                let mut new_group = group.clone();
                new_group.push(node);
                to_add.push(new_group);
            }
        }
        groups.extend(to_add.drain(..));
    }

    let largest_group = groups.into_iter().max_by_key(|group| group.len()).unwrap();
    let result = largest_group
        .into_iter()
        .map(|node| graph[node])
        .sorted()
        .join(",");
    println!("Result: {result}");
}
