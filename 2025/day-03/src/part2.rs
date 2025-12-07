use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let banks = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut sum = 0;
    for bank in banks {
        let mut remaining = &bank[..];
        let mut joltage = 0;
        for i in 1..=12 {
            let left = 12 - i;
            let pos = remaining.len()
                - remaining
                    .iter()
                    .take(remaining.len() - left)
                    .rev()
                    .position_max()
                    .unwrap()
                - left
                - 1;

            joltage = joltage * 10 + remaining[pos] as u64;
            remaining = &remaining[(pos + 1)..];
        }

        sum += joltage;
    }

    println!("Result: {}", sum);
}
