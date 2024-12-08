use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    let data = input.chars().filter(|c| !c.is_whitespace()).collect_vec();
    let height = input.lines().filter(|s| !s.is_empty()).count() as i32;
    let width = data.len() as i32 / height;
    let groups = data
        .into_iter()
        .enumerate()
        .filter(|(_, c)| *c != '.')
        .map(|(index, c)| ((index as i32 % width, index as i32 / width), c))
        .into_group_map_by(|(_, v)| *v);

    let mut antinodes = HashSet::new();
    for (_, antennas) in groups.iter() {
        for (pos, _) in antennas.iter() {
            for (other, _) in antennas.iter() {
                if pos == other {
                    continue;
                }
                let antinode = (2 * pos.0 - other.0, 2 * pos.1 - other.1);
                if antinode.0 >= 0 && antinode.0 < width && antinode.1 >= 0 && antinode.1 < height {
                    antinodes.insert(antinode);
                }
            }
        }
    }
    println!("Result: {}", antinodes.len());
}
