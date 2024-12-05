use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let str = include_str!("../input.txt");

    let mut lines = str.lines();
    let rules: Vec<(u8, u8)> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.split("|")
                .map(|x| x.parse::<u8>().unwrap())
                .next_tuple()
                .unwrap()
        })
        .collect_vec();

    let updates = lines
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<u8>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let rules: HashSet<(u8, u8)> = rules.into_iter().collect();

    let mut sum = 0;
    'next_update: for update in updates {
        for i in 0..update.len() {
            for j in i..update.len() {
                if rules.contains(&(update[j], update[i])) {
                    continue 'next_update;
                }
            }
        }
        sum += update[(update.len() - 1) / 2] as usize;
    }

    println!("Result: {sum}");
}
