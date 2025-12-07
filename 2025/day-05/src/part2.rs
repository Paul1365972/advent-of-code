use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let (input1, _) = input.trim().split_once("\n\n").unwrap();

    let ranges: Vec<(u64, u64)> = input1
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect_vec();

    let mut range_set: Vec<(u64, u64)> = Vec::new();

    for range in ranges {
        range_set.push(range);
        let hits_pos = range_set
            .iter()
            .positions(|other| range.0 <= other.1 && other.0 <= range.1)
            .collect_vec();

        let hits = hits_pos.iter().map(|&index| range_set[index]).collect_vec();
        let min = *hits.iter().map(|(value, _)| value).min().unwrap();
        let max = *hits.iter().map(|(_, value)| value).max().unwrap();
        for &pos in hits_pos.iter().rev() {
            range_set.swap_remove(pos);
        }
        range_set.push((min, max));
    }

    let result = range_set
        .into_iter()
        .map(|(start, end)| end - start + 1)
        .sum::<u64>();

    println!("Result: {}", result);
}
