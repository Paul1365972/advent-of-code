use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");

    let mut map = input
        .lines()
        .take_while(|line| !line.is_empty())
        .flat_map(|line| line.chars())
        .collect_vec();
    let moves = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .flat_map(|line| line.chars())
        .collect_vec();

    let height = input.lines().take_while(|s| !s.is_empty()).count() as i32;
    let width = map.len() as i32 / height;
    let start_index = map.iter().position(|c| *c == '@').unwrap();
    map[start_index] = '.';

    let mut pos = ((start_index as i32 % width), (start_index as i32 / width));

    for mov in moves {
        let dir = match mov {
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            _ => unreachable!(),
        };
        let mut length = 1;
        loop {
            let c = map[to_index(pos, dir, length, width)];
            match c {
                '.' => {
                    map.swap(
                        to_index(pos, dir, 1, width),
                        to_index(pos, dir, length, width),
                    );
                    pos = (pos.0 + dir.0, pos.1 + dir.1);
                    break;
                }
                'O' => {
                    length += 1;
                }
                '#' => {
                    break;
                }
                _ => unreachable!(),
            }
        }
    }

    let sum: usize = map
        .into_iter()
        .enumerate()
        .filter(|(_, c)| *c == 'O')
        .map(|(i, _)| i % width as usize + i / width as usize * 100)
        .sum();
    println!("Result: {sum}");
}

fn to_index(pos: (i32, i32), dir: (i32, i32), length: i32, width: i32) -> usize {
    ((pos.0 + dir.0 * length) + (pos.1 + dir.1 * length) * width) as usize
}
