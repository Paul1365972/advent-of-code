use itertools::Itertools;
use rustc_hash::FxHashSet;

const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn main() {
    let input = include_str!("../input.txt");

    let mut data = input.chars().filter(|c| !c.is_whitespace()).collect_vec();
    let height = input.lines().filter(|s| !s.is_empty()).count();
    let width = data.len() / height;
    let start_index = data.iter().position(|c| *c == '^').unwrap();
    data[start_index] = '.';

    let mut pos = ((start_index % width) as i32, (start_index / width) as i32);
    let mut dir = 0;
    let mut visited = FxHashSet::from_iter([]);
    loop {
        let ahead_pos = (pos.0 + DIRS[dir].0, pos.1 + DIRS[dir].1);
        let ahead = get(&data, width, height, ahead_pos.0, ahead_pos.1);
        match ahead {
            Some('.') => {
                pos = ahead_pos;
                visited.insert(pos);
            }
            Some('#') => dir = (dir + 1) % 4,
            None => break,
            _ => unreachable!(),
        };
    }

    let mut valid_blockers = 0;
    for blocker in visited {
        let mut pos = ((start_index % width) as i32, (start_index / width) as i32);
        let mut dir = 0;
        let mut visited = FxHashSet::from_iter([(pos, dir)]);
        loop {
            let ahead_pos = (pos.0 + DIRS[dir].0, pos.1 + DIRS[dir].1);
            let ahead = if ahead_pos != blocker {
                get(&data, width, height, ahead_pos.0, ahead_pos.1)
            } else {
                Some('#')
            };
            match ahead {
                Some('.') => pos = ahead_pos,
                Some('#') => dir = (dir + 1) % 4,
                None => break,
                _ => unreachable!(),
            };
            if !visited.insert((pos, dir)) {
                valid_blockers += 1;
                break;
            }
        }
    }

    println!("Result: {valid_blockers}");
}

fn get(data: &[char], width: usize, height: usize, x: i32, y: i32) -> Option<char> {
    return if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
        Some(data[x as usize + y as usize * width])
    } else {
        None
    };
}
