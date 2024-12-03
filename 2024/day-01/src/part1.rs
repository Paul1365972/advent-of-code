use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    let mut left = vec![];
    let mut right = vec![];
    for mut row in &input
        .split_ascii_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .chunks(2)
    {
        left.push(row.next().unwrap());
        right.push(row.next().unwrap());
    }
    left.sort();
    right.sort();
    let result: u32 = left
        .into_iter()
        .zip_eq(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum();
    println!("Result: {}", result);
}
