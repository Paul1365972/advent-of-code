use itertools::Itertools;
use rustc_hash::FxHashSet;

fn main() {
    let input = include_str!("../input.txt");

    let mut lines = input.lines();
    let patterns = lines.next().unwrap().split(", ").collect_vec();
    let designs = lines.filter(|line| !line.is_empty()).collect_vec();

    let mut designs_possible = 0;
    for design in designs {
        let mut matched = FxHashSet::from_iter(patterns.iter().map(|str| str.to_string()));
        let mut buffer = FxHashSet::default();
        let mut temp = String::new();
        while !matched.is_empty() && !matched.contains(design) {
            for partial in matched.drain() {
                for pattern in patterns.iter().copied() {
                    temp.clear();
                    temp.push_str(&partial);
                    temp.push_str(&pattern);
                    if design.starts_with(&temp) {
                        buffer.insert(temp.clone());
                    }
                }
            }
            std::mem::swap(&mut matched, &mut buffer);
        }
        if matched.contains(design) {
            designs_possible += 1;
        }
    }

    println!("Result: {designs_possible}");
}
