use std::{ops::Range, str::FromStr};

use itertools::Itertools;
use rustc_hash::FxHashMap;

fn main() {
    const OUTPUTS: Range<usize> = 0..46;
    let input = include_str!("../input.txt");

    let inital_values: FxHashMap<&str, bool> = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| (&line[0..3], line[5..6].parse::<u8>().unwrap() != 0))
        .collect();

    let connections: FxHashMap<&str, (Operation, &str, &str)> = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            let parts = line.split_ascii_whitespace().collect_vec();
            let operation = Operation::from_str(parts[1]).unwrap();
            (parts[4], (operation, parts[0], parts[2]))
        })
        .collect();

    let mut values = inital_values.clone();

    let mut result: u64 = 0;
    for i in OUTPUTS {
        let node = format!("z{i:0>2}");
        let value = get_value(&node, &mut values, &connections);
        result |= value as u64 * (1 << i);
    }
    println!("Result: {result}");
}

fn get_value<'a>(
    node: &str,
    values: &mut FxHashMap<&'a str, bool>,
    connections: &FxHashMap<&'a str, (Operation, &'a str, &'a str)>,
) -> bool {
    if let Some(value) = values.get(node) {
        return *value;
    } else {
        let (node, (op, a, b)) = connections.get_key_value(node).unwrap();
        let a = get_value(a, values, connections);
        let b = get_value(b, values, connections);
        let value = match op {
            Operation::And => a & b,
            Operation::Or => a | b,
            Operation::Xor => a ^ b,
        };
        values.insert(node, value);
        return value;
    }
}

#[derive(strum::EnumString, Clone, Copy)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
enum Operation {
    And,
    Or,
    Xor,
}
