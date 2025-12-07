use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let (input1, input2) = input.trim().split_once("\n\n").unwrap();

    let ranges: Vec<(u64, u64)> = input1
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect_vec();

    let ids: Vec<u64> = input2
        .lines()
        .map(|line| line.parse().unwrap())
        .collect_vec();

    let mut sum = 0;

    for id in ids {
        let fresh = ranges.iter().any(|range| (range.0..=range.1).contains(&id));
        if fresh {
            sum += 1;
        }
    }

    println!("Result: {}", sum);
}
