use itertools::Itertools;
use rand::Rng;
use rustc_hash::FxHashMap;
use std::str::FromStr;

fn main() {
    const INPUTS: usize = 45;
    const OUTPUTS: usize = 46;

    let input = include_str!("../input.txt");

    let connections: FxHashMap<String, (Operation, String, String)> = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            let parts = line.split_ascii_whitespace().collect_vec();
            let operation = Operation::from_str(parts[1]).unwrap();
            (
                parts[4].to_owned(),
                (operation, parts[0].to_owned(), parts[2].to_owned()),
            )
        })
        .collect();

    let mut rand = rand::thread_rng();

    let mut values = FxHashMap::default();
    let mut badness = FxHashMap::default();

    for _ in 0..10_000 {
        values.clear();
        let x: u64 = rand.gen_range(0..1 << INPUTS);
        let y: u64 = rand.gen_range(0..1 << INPUTS);
        let z = x + y;
        for i in 0..INPUTS {
            values.insert(format!("x{i:0>2}"), (x & (1 << i)) != 0);
            values.insert(format!("y{i:0>2}"), (x & (1 << i)) != 0);
        }
        for i in 0..OUTPUTS {
            let node = format!("z{i:0>2}");
            let value = get_value(&node, &mut values, &connections);
            let expected = z & (1 << i) != 0;
            if value != expected {
                increase_badness(&node, 1.0, &mut badness, &connections);
            }
        }
    }

    let result = badness
        .into_iter()
        .sorted_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .take(8)
        .map(|(node, _)| node)
        .sorted()
        .join(",");
    println!("Resuilt: {result}");
}

fn increase_badness(
    node: &str,
    score: f64,
    badness: &mut FxHashMap<String, f64>,
    connections: &FxHashMap<String, (Operation, String, String)>,
) {
    if let Some(badness) = badness.get_mut(node) {
        *badness += score;
    } else {
        badness.insert(node.to_owned(), score);
    }
    if let Some((_, a, b)) = connections.get(node) {
        increase_badness(a, score / 2.0, badness, connections);
        increase_badness(b, score / 2.0, badness, connections);
    }
}

fn get_value(
    node: &str,
    values: &mut FxHashMap<String, bool>,
    connections: &FxHashMap<String, (Operation, String, String)>,
) -> bool {
    if let Some(value) = values.get(node) {
        return *value;
    } else {
        let (op, a, b) = connections.get(node).unwrap();
        let a = get_value(&a, values, connections);
        let b = get_value(b, values, connections);
        let value = match op {
            Operation::And => a & b,
            Operation::Or => a | b,
            Operation::Xor => a ^ b,
        };
        values.insert(node.to_owned(), value);
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
