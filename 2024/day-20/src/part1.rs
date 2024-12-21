use itertools::Itertools;
use rustc_hash::FxHashMap;

fn main() {
    let input = include_str!("../input.txt");

    let map = input.lines().flat_map(|line| line.chars()).collect_vec();

    let height = input.lines().filter(|s| !s.is_empty()).count() as isize;
    let width = map.len() as isize / height;

    let start_index = map.iter().position(|c| *c == 'S').unwrap() as isize;
    let end_index = map.iter().position(|c| *c == 'E').unwrap() as isize;

    let start_pos = ((start_index % width), (start_index / width));
    let end_pos = ((end_index % width), (end_index / width));

    let mut path = FxHashMap::from_iter([(start_pos, 0)]);
    let mut pos = start_pos;
    while pos != end_pos {
        for offset in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let next_pos = (pos.0 + offset.0, pos.1 + offset.1);

            if get(&map, width, height, next_pos) != Some('#') && !path.contains_key(&next_pos) {
                path.insert(next_pos, path.len() as isize);
                pos = next_pos;
                break;
            }
        }
    }

    let mut shortcuts: FxHashMap<_, usize> = FxHashMap::default();
    for (pos, distance) in path.iter() {
        for offset in [
            (2, 0),
            (1, 1),
            (0, 2),
            (-1, 1),
            (-2, 0),
            (-1, -1),
            (0, -2),
            (1, -1),
        ] {
            let next_pos = (pos.0 + offset.0, pos.1 + offset.1);

            if let Some(other_distance) = path.get(&next_pos) {
                let improvement = *distance - *other_distance - 2;
                *shortcuts.entry(improvement).or_default() += 1;
            }
        }
    }

    let good_cheats: usize = shortcuts
        .into_iter()
        .filter(|(improvement, _)| *improvement >= 100)
        .map(|(_, count)| count)
        .sum();

    println!("Result: {good_cheats}");
}

fn get(map: &[char], width: isize, height: isize, pos: (isize, isize)) -> Option<char> {
    if pos.0 < 0 || pos.0 >= width || pos.1 < 0 || pos.1 >= height {
        return None;
    }
    return Some(map[pos.0 as usize + pos.1 as usize * width as usize]);
}
