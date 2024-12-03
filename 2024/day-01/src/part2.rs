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
    let right = right.into_iter().counts();
    let result: u32 = left
        .into_iter()
        .map(|x| x * *right.get(&x).unwrap_or(&0) as u32)
        .sum();
    println!("Result: {}", result);
}
