use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    let initial_prices = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect_vec();

    let mut result = 0;
    for mut price in initial_prices.iter().copied() {
        for _ in 0..2000 {
            price = next_random_number(price);
        }
        result += price;
    }

    println!("Result: {result}");
}

fn next_random_number(num: u64) -> u64 {
    let num = ((num * 64) ^ num) % 16777216;
    let num = ((num / 32) ^ num) % 16777216;
    let num = ((num * 2048) ^ num) % 16777216;
    return num;
}
