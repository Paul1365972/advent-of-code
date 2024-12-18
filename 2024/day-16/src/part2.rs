use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{collections::BinaryHeap, isize};

fn main() {
    let input = include_str!("../input.txt");

    let map = input.lines().flat_map(|line| line.chars()).collect_vec();

    let height = input.lines().filter(|s| !s.is_empty()).count() as i32;
    let width = map.len() as i32 / height;

    let start_index = map.iter().position(|c| *c == 'S').unwrap();
    let end_index = map.iter().position(|c| *c == 'E').unwrap();

    let start_pos = ((start_index as i32 % width), (start_index as i32 / width));
    let end_pos = ((end_index as i32 % width), (end_index as i32 / width));

    // Dijkstra's Algorithm
    let mut best = FxHashMap::default();
    let mut prev: FxHashMap<_, Vec<_>> = FxHashMap::default();
    let mut heap = BinaryHeap::new();
    heap.push((0, (start_pos, Direction::East)));
    while let Some((score, (pos, dir))) = heap.pop() {
        if score < *best.get(&(pos, dir)).unwrap_or(&isize::MIN) {
            continue;
        }
        let original_state = (pos, dir);
        let forward_pos = (pos.0 + dir.to_dir().0, pos.1 + dir.to_dir().1);
        for (score, (pos, dir)) in [
            (score - 1, (forward_pos, dir)),
            (score - 1000, (pos, dir.rotations()[0])),
            (score - 1000, (pos, dir.rotations()[1])),
        ] {
            if map[(pos.0 + pos.1 * width) as usize] != '#' {
                let best_score = best.entry((pos, dir)).or_insert(isize::MIN);
                if *best_score <= score {
                    *best_score = score;
                    heap.push((score, (pos, dir)));
                    prev.entry((pos, dir)).or_default().push(original_state);
                }
            }
        }
    }

    // DFS for all paths then deduplicate
    let mut visited = FxHashSet::default();
    let mut stack = Vec::from_iter(Direction::ALL.map(|dir| (end_pos, dir)));
    while let Some(state) = stack.pop() {
        if visited.insert(state) {
            stack.extend(prev.get(&state).unwrap_or(&Vec::new()));
        }
    }
    let good_spots = FxHashSet::from_iter(visited.into_iter().map(|(pos, _)| pos));

    println!("Result: {}", good_spots.len());
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    East,
    South,
    West,
    North,
}

impl Direction {
    const ALL: [Direction; 4] = [
        Direction::East,
        Direction::South,
        Direction::West,
        Direction::North,
    ];

    fn to_dir(&self) -> (i32, i32) {
        match self {
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::North => (0, -1),
        }
    }

    fn rotations(&self) -> [Direction; 2] {
        match self {
            Direction::East => [Direction::North, Direction::South],
            Direction::South => [Direction::East, Direction::West],
            Direction::West => [Direction::South, Direction::North],
            Direction::North => [Direction::West, Direction::East],
        }
    }
}
