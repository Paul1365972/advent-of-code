use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();

    let mut inputs = input
        .trim()
        .lines()
        .map(|line| line.split_ascii_whitespace().collect_vec())
        .collect_vec();

    let problems = inputs
        .iter()
        .map(|line| line.len())
        .all_equal_value()
        .unwrap();
    let operations = inputs.pop().unwrap();

    let mut sum = 0;

    for index in 0..problems {
        let numbers = inputs
            .iter()
            .map(|line| line[index].parse::<u64>().unwrap());

        let result = match operations[index] {
            "+" => numbers.sum::<u64>(),
            "*" => numbers.product::<u64>(),
            _ => unreachable!(),
        };
        sum += result;
    }

    println!("Result: {}", sum);
}
