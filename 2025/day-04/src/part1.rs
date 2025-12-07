use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Roll,
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let height = input.trim().lines().count() as i32;
    let board = input
        .trim()
        .lines()
        .flat_map(|line| line.chars())
        .map(|char| match char {
            '.' => Tile::Empty,
            '@' => Tile::Roll,
            _ => unreachable!(),
        })
        .collect_vec();
    let width = board.len() as i32 / height;

    let mut sum = 0usize;

    for y in 0..height {
        for x in 0..width {
            let tile = get(&board, width, height, (x, y)).unwrap();
            if tile != Tile::Roll {
                continue;
            }

            let mut neighbors = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let tile = get(&board, width, height, (x + dx, y + dy)).unwrap_or(Tile::Empty);
                    if tile == Tile::Roll {
                        neighbors += 1;
                    }
                }
            }
            if neighbors - 1 < 4 {
                sum += 1;
            }
        }
    }

    println!("Result: {}", sum);
}

fn get(board: &Vec<Tile>, width: i32, height: i32, pos: (i32, i32)) -> Option<Tile> {
    if pos.0 < 0 || pos.0 >= width || pos.1 < 0 || pos.1 >= height {
        return None;
    }
    return Some(board[(pos.0 + pos.1 * width) as usize]);
}
