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
        let first_pos = bank.len()
            - bank
                .iter()
                .take(bank.len() - 1)
                .rev()
                .position_max()
                .unwrap()
            - 2;
        let second_pos = bank.len()
            - bank
                .iter()
                .skip(first_pos + 1)
                .rev()
                .position_max()
                .unwrap()
            - 1;

        let joltage = bank[first_pos] * 10 + bank[second_pos];

        sum += joltage;
    }

    println!("Result: {}", sum);
}
