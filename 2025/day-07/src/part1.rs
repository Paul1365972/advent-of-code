use itertools::Itertools;
use rustc_hash::FxHashSet;

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

    let mut total_splits = 0;
    let mut beams = FxHashSet::default();
    beams.insert(start);
    for splitters in splitter_levels {
        for splitter in splitters {
            if beams.contains(&splitter) {
                total_splits += 1;
                beams.insert(splitter + 1);
                beams.insert(splitter - 1);
                beams.remove(&splitter);
            }
        }
    }

    println!("Result: {}", total_splits);
}
