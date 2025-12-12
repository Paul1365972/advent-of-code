use std::collections::VecDeque;

use itertools::Itertools;
use rustc_hash::FxHashMap;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();

    let tiles: Vec<(i64, i64)> = input
        .trim()
        .lines()
        .map(|line| {
            line.split(",")
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    let x_values: FxHashMap<i64, usize> = tiles
        .iter()
        .map(|(x, _)| *x)
        .sorted_unstable()
        .dedup()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect();
    let y_values: FxHashMap<i64, usize> = tiles
        .iter()
        .map(|(_, y)| *y)
        .sorted_unstable()
        .dedup()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect();

    let compressed = tiles
        .iter()
        .map(|(x, y)| (*x_values.get(x).unwrap(), *y_values.get(y).unwrap()))
        .collect_vec();

    let mut board = vec![false; x_values.len() * y_values.len()];

    let mut cursor = compressed.last().unwrap();
    for tile in compressed.iter() {
        let min_x = tile.0.min(cursor.0);
        let max_x = tile.0.max(cursor.0);
        let min_y = tile.1.min(cursor.1);
        let max_y = tile.1.max(cursor.1);
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                board[x + y * x_values.len()] = true;
            }
        }
        cursor = tile;
    }

    let mut queue = VecDeque::new();
    queue.push_back((125, 125));
    while let Some(pos) = queue.pop_front() {
        if board[pos.0 + pos.1 * x_values.len()] {
            continue;
        }
        board[pos.0 + pos.1 * x_values.len()] = true;
        for pos in [
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1),
        ] {
            queue.push_back(pos);
        }
    }

    let (a, b) = (0..tiles.len())
        .flat_map(|a| (0..a).map(move |b| (a, b)))
        .sorted_unstable_by_key(|(a, b)| size(tiles[*a], tiles[*b]))
        .rev()
        .filter(|(a, b)| {
            let a = &compressed[*a];
            let b = &compressed[*b];
            let min_x = a.0.min(b.0);
            let max_x = a.0.max(b.0);
            let min_y = a.1.min(b.1);
            let max_y = a.1.max(b.1);
            (min_x..=max_x)
                .cartesian_product(min_y..=max_y)
                .all(|(x, y)| board[x + y * x_values.len()])
        })
        .next()
        .unwrap();

    let result = size(tiles[a], tiles[b]);

    println!("Result: {}", result);
}

fn size(a: (i64, i64), b: (i64, i64)) -> i64 {
    return ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1);
}
