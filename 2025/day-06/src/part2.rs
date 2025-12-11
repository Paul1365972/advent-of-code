use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut lines = input.lines().filter(|line| !line.is_empty()).collect_vec();

    let total_length = lines
        .iter()
        .map(|line| line.len())
        .all_equal_value()
        .unwrap();
    let operations = lines.pop().unwrap();

    let mut sum = 0;
    let mut operation = "?";
    let mut numbers = Vec::new();
    for i in 0..total_length {
        if lines.iter().map(|line| &line[i..=i]).all(|s| s == " ") {
            sum += calc(numbers.drain(..), operation);
            operation = "?";
            continue;
        }
        let maybe_operation = &operations[i..=i];
        if maybe_operation != " " {
            operation = maybe_operation;
        }
        let num = lines
            .iter()
            .map(|line| &line[i..=i])
            .join("")
            .trim()
            .parse()
            .unwrap();
        numbers.push(num);
    }
    sum += calc(numbers.drain(..), operation);

    println!("Result: {}", sum);
}

fn calc(numbers: impl IntoIterator<Item = u64>, operation: &str) -> u64 {
    match operation {
        "+" => numbers.into_iter().sum::<u64>(),
        "*" => numbers.into_iter().product::<u64>(),
        _ => unreachable!(),
    }
}
