use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::collections::BinaryHeap;

const WIDTH: i8 = 71;
const HEIGHT: i8 = 71;

fn main() {
    let input = include_str!("../input.txt");

    let falling_coords = input
        .lines()
        .map(|str| {
            let (x, y) = str.split_once(",").unwrap();
            (x.parse::<u8>().unwrap(), y.parse::<u8>().unwrap())
        })
        .collect_vec();

    let mut map = [[false; WIDTH as usize]; HEIGHT as usize];
    for falling in falling_coords.iter() {
        map[falling.0 as usize][falling.1 as usize] = true;
        if dijkstra(map).is_none() {
            println!("Result: {},{}", falling.0, falling.1);
            break;
        }
    }
}

fn dijkstra(map: [[bool; WIDTH as usize]; HEIGHT as usize]) -> Option<isize> {
    let start_pos: (i8, i8) = (0, 0);
    let target_pos: (i8, i8) = (WIDTH - 1, HEIGHT - 1);

    // Dijkstra's Algorithm
    let mut best = FxHashMap::default();
    let mut heap = BinaryHeap::new();
    heap.push((0, start_pos));
    while let Some((score, pos)) = heap.pop() {
        if score < *best.get(&pos).unwrap_or(&isize::MIN) {
            continue;
        }

        for offset in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let pos = (pos.0 + offset.0, pos.1 + offset.1);
            let score = score - 1;
            if pos.0 >= 0 && pos.0 < WIDTH && pos.1 >= 0 && pos.1 < HEIGHT {
                if map[pos.0 as usize][pos.1 as usize] == false {
                    let best_score = best.entry(pos).or_insert(isize::MIN);
                    if *best_score < score {
                        *best_score = score;
                        heap.push((score, pos));
                    }
                }
            }
        }
    }

    return best.get(&target_pos).map(|score| -*score);
}
