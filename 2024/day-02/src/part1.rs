use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    let reports = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let result = reports
        .into_iter()
        .filter(|report| {
            let range = if report[0] < report[1] {
                1..=3
            } else {
                -3..=-1
            };
            report
                .iter()
                .tuple_windows()
                .all(|(a, b)| range.contains(&(b - a)))
        })
        .count();
    println!("Result: {}", result);
}
