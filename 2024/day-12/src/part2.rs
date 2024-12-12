use itertools::Itertools;
use rustc_hash::FxHashSet;

const DIRS: [(isize, isize); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

fn main() {
    let kernel_map = create_kernel_map();

    let input = include_str!("../input.txt");

    let data = input.chars().filter(|c| !c.is_whitespace()).collect_vec();
    let height = input.lines().filter(|s| !s.is_empty()).count() as isize;
    let width = data.len() as isize / height;

    let mut cost: u64 = 0;

    let mut visited = FxHashSet::default();
    for start_y in 0..height {
        for start_x in 0..width {
            if visited.contains(&(start_x, start_y)) {
                continue;
            }
            let mut area = 0;
            let mut edges = 0;
            let identifier = get(&data, width, height, start_x, start_y).unwrap();
            let mut stack = Vec::from_iter([(start_x, start_y)]);
            while let Some(pos) = stack.pop() {
                if visited.contains(&pos) {
                    continue;
                }
                visited.insert(pos);
                area += 1;
                for dir in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
                    let neighbor = get(&data, width, height, pos.0 + dir.0, pos.1 + dir.1);
                    if neighbor == Some(identifier) {
                        stack.push((pos.0 + dir.0, pos.1 + dir.1));
                    }
                }
                let mut state = 0;
                for (i, dir) in DIRS.into_iter().enumerate() {
                    let neighbor = get(&data, width, height, pos.0 + dir.0, pos.1 + dir.1);
                    if neighbor == Some(identifier) {
                        state |= 1 << i;
                    }
                }
                edges += kernel_map[state] as u64;
            }
            cost += area * edges;
        }
    }

    println!("Result: {cost}");
}

fn get(data: &[char], width: isize, height: isize, x: isize, y: isize) -> Option<char> {
    return if x >= 0 && x < width && y >= 0 && y < height {
        Some(data[x as usize + y as usize * width as usize])
    } else {
        None
    };
}

fn create_kernel_map() -> [u8; 256] {
    let mut kernel_map = [0; 256];
    for (state, edges) in kernel_map.iter_mut().enumerate() {
        for (left, edge, right) in (0..8).into_iter().circular_tuple_windows().step_by(2) {
            let left = state & (1 << left) != 0;
            let edge = state & (1 << edge) != 0;
            let right = state & (1 << right) != 0;
            if left && !edge && right || !left && !right {
                *edges += 1;
            }
        }
    }

    return kernel_map;
}
