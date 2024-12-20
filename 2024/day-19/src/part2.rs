use itertools::Itertools;
use rustc_hash::FxHashMap;

fn main() {
    let input = include_str!("../input.txt");

    let mut lines = input.lines();
    let patterns = lines.next().unwrap().split(", ").collect_vec();
    let designs = lines.filter(|line| !line.is_empty()).collect_vec();

    let mut result = 0usize;
    for design in designs {
        let mut matched = FxHashMap::default();
        let mut buffer = FxHashMap::default();
        matched.insert("".to_owned(), 1usize);
        let mut temp = String::new();
        while !matched.is_empty() {
            for (partial, possibilities) in matched.drain() {
                for pattern in patterns.iter().copied() {
                    temp.clear();
                    temp.push_str(&partial);
                    temp.push_str(&pattern);
                    if design.starts_with(&temp) {
                        *buffer.entry(temp.clone()).or_default() += possibilities;
                    }
                    if design == temp {
                        result += possibilities;
                    }
                }
            }
            std::mem::swap(&mut matched, &mut buffer);
        }
    }

    println!("Result: {result}");
}
