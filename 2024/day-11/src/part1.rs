use itertools::Itertools;
use std::mem::swap;

fn main() {
    let input = include_str!("../input.txt");

    let mut data = input
        .split_ascii_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect_vec();

    let mut buffer = Vec::new();
    for _ in 1..=25 {
        for stone in data.drain(..) {
            let stone_str = stone.to_string();
            if stone == 0 {
                buffer.push(1);
            } else if stone_str.len() % 2 == 0 {
                buffer.push(stone_str[..stone_str.len() / 2].parse::<u64>().unwrap());
                buffer.push(stone_str[stone_str.len() / 2..].parse::<u64>().unwrap());
            } else {
                buffer.push(stone * 2024);
            }
        }
        swap(&mut data, &mut buffer);
    }

    println!("Result: {}", data.len());
}
