use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let ranges = input
        .trim()
        .split(",")
        .map(|range| {
            let [start, end] = range.split("-").collect_array().unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect_vec();

    let mut sum = 0;
    for (start, end) in ranges {
        sum += count_invalids_in_range(start, end);
    }

    println!("Result: {}", sum);
}

fn count_invalids_in_range(start: u64, end: u64) -> u64 {
    let mut sum = 0;

    println!("Check range: {start}-{end}");

    for length in (start.ilog10() + 1)..=(end.ilog10() + 1) {
        if length % 2 != 0 {
            continue;
        }

        let half = 10u64.pow(length / 2);
        let lower = 10u64.pow(length - 1);
        let upper = 10u64.pow(length) - 1;

        let start_prefix = start.max(lower) / half;
        let end_prefix = end.min(upper) / half;

        for prefix in start_prefix..=end_prefix {
            let value = prefix * half + prefix;
            if (start..=end).contains(&value) {
                sum += value;
                println!("Found value: {value}");
            }
        }
    }

    return sum;
}
