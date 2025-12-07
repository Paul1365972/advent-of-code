use itertools::Itertools;
use rustc_hash::FxHashSet;
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

    let mut found = FxHashSet::default();
    for (start, end) in ranges {
        let new = count_invalids_in_range(start, end);
        found.extend(new);
    }

    let result = found.into_iter().sum::<u64>();
    println!("Result: {}", result);
}

fn count_invalids_in_range(start: u64, end: u64) -> FxHashSet<u64> {
    let mut found = FxHashSet::default();

    println!("Check range: {start}-{end}");

    for length in (start.ilog10() + 1)..=(end.ilog10() + 1) {
        for segments in 2..=length {
            if length % segments != 0 {
                continue;
            }

            let segment_size = 10u64.pow(length / segments);
            let segment_div = 10u64.pow(length - length / segments);
            let lower = 10u64.pow(length - 1);
            let upper = 10u64.pow(length) - 1;

            let start_prefix = start.max(lower) / segment_div;
            let end_prefix = end.min(upper) / segment_div;

            for prefix in start_prefix..=end_prefix {
                let mut value = 0;
                for _ in 0..segments {
                    value = value * segment_size + prefix;
                }
                if (start..=end).contains(&value) {
                    if found.insert(value) {
                        println!("Found value: {value}");
                    }
                }
            }
        }
    }

    return found;
}
