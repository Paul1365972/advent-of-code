use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::iter::successors;

fn main() {
    let input = include_str!("../input.txt");
    let initial_numbers = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect_vec();

    let mut prices: FxHashMap<(i8, i8, i8, i8), u64> = FxHashMap::default();

    for initial in initial_numbers {
        let mut used = FxHashSet::default();
        for window in successors(Some(initial), |&x| Some(next_random_number(x)))
            .take(2000)
            .map(|x| (x % 10) as i8)
            .tuple_windows::<(i8, i8, i8, i8, i8)>()
        {
            let pattern = (
                window.1 - window.0,
                window.2 - window.1,
                window.3 - window.2,
                window.4 - window.3,
            );
            if used.insert(pattern) {
                *prices.entry(pattern).or_default() += window.4 as u64;
            }
        }
    }

    let result = *prices.values().max().unwrap();
    println!("Result: {result}");
}

fn next_random_number(num: u64) -> u64 {
    let num = ((num * 64) ^ num) % 16777216;
    let num = ((num / 32) ^ num) % 16777216;
    let num = ((num * 2048) ^ num) % 16777216;
    return num;
}
