use itertools::Itertools;
use rustc_hash::FxHashSet;

fn main() {
    let input = include_str!("../input.txt");

    let equations = input
        .lines()
        .map(|line| {
            let mut split = line.split(": ");
            (
                split.next().unwrap().parse::<u64>().unwrap(),
                split
                    .next()
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|num| num.parse::<u64>().unwrap())
                    .collect_vec(),
            )
        })
        .collect_vec();

    let mut sum = 0;
    for (result, mut nums) in equations {
        let mut solutions = FxHashSet::from_iter([nums.remove(0)]);
        for num in nums {
            for solution in solutions.drain().collect_vec() {
                solutions.insert(solution + num);
                solutions.insert(solution * num);
            }
        }
        if solutions.contains(&result) {
            sum += result;
        }
    }
    println!("Result: {sum}");
}
