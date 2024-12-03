use itertools::Itertools;

fn check_safe(report: &Vec<i32>) -> bool {
    let range = if report[0] < report[1] {
        1..=3
    } else {
        -3..=-1
    };
    report
        .into_iter()
        .tuple_windows()
        .all(|(a, b)| range.contains(&(b - a)))
}

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
            check_safe(report)
                || (0..report.len())
                    .map(|i| {
                        let mut report = report.clone();
                        report.remove(i);
                        report
                    })
                    .any(|report| check_safe(&report))
        })
        .count();
    println!("Result: {}", result);
}
