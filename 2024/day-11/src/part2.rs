use rustc_hash::FxHashMap;
use std::mem::swap;

fn main() {
    let input = include_str!("../input.txt");

    let mut data: FxHashMap<u64, u64> = FxHashMap::default();
    input
        .split_ascii_whitespace()
        .map(|str| str.parse().unwrap())
        .for_each(|num| *data.entry(num).or_default() += 1);

    let mut buffer = FxHashMap::default();
    for _ in 1..=75 {
        for (stone, amount) in data.drain() {
            let stone_str = stone.to_string();
            if stone == 0 {
                *buffer.entry(1).or_default() += amount;
            } else if stone_str.len() % 2 == 0 {
                let mid = stone_str.len() / 2;
                *buffer.entry(stone_str[..mid].parse().unwrap()).or_default() += amount;
                *buffer.entry(stone_str[mid..].parse().unwrap()).or_default() += amount;
            } else {
                *buffer.entry(stone * 2024).or_default() += amount;
            }
        }
        swap(&mut data, &mut buffer);
    }

    println!("Result: {}", data.into_values().sum::<u64>());
}
