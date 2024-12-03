use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");

    let disabled_regions_regex = Regex::new(r"don't\(\)[\S\s]*?(do\(\)|$)").unwrap();
    let fixed_input = disabled_regions_regex.replace_all(input, "");

    let mul_regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let num_digits = 1..=3usize;
    let result: u32 = mul_regex
        .captures_iter(&fixed_input)
        .map(|m| (m.get(1).unwrap().as_str(), m.get(2).unwrap().as_str()))
        .filter(|(a, b)| num_digits.contains(&a.len()) && num_digits.contains(&b.len()))
        .map(|(a, b)| a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap())
        .sum();

    println!("Result: {result}")
}