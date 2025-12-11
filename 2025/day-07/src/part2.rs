use itertools::Itertools;
use rustc_hash::FxHashMap;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();

    let start = input.find("S").unwrap();

    let splitter_levels= input
        .trim()
        .lines()
        .map(|line| {
            line.char_indices()
                .filter_map(|(index, char)| if char == '^' { Some(index) } else { None })
                .collect_vec()
        })
        .collect_vec();

    let mut beams = FxHashMap::default();
    beams.insert(start, 1);
    for splitters in splitter_levels {
        for splitter in splitters {
            if let Some(&timelines) = beams.get(&splitter) {
                *beams.entry(splitter + 1).or_default() += timelines;
                *beams.entry(splitter - 1).or_default() += timelines;
                beams.remove(&splitter);
            }
        }
    }

    let result = beams.into_values().sum::<u64>();
    println!("Result: {}", result);
}
