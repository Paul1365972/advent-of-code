use itertools::Itertools;
use rustc_hash::FxHashSet;

fn main() {
    let input = include_str!("../input.txt");

    let mut map = input
        .lines()
        .take_while(|line| !line.is_empty())
        .flat_map(|line| line.chars())
        .flat_map(|c| match c {
            '#' => ['#', '#'],
            'O' => ['[', ']'],
            '.' => ['.', '.'],
            '@' => ['@', '.'],
            _ => unreachable!(),
        })
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
        let mut to_move = FxHashSet::default();
        let mut valid = true;
        let mut stack = Vec::from_iter([(pos.0 + dir.0, pos.1 + dir.1)]);
        while let Some(item) = stack.pop() {
            if to_move.contains(&item) {
                continue;
            }
            let c = map[(item.0 + item.1 * width) as usize];
            match c {
                '.' => {}
                '[' => {
                    to_move.insert(item);
                    stack.push((item.0 + 1, item.1));
                    stack.push((item.0 + dir.0, item.1 + dir.1));
                }
                ']' => {
                    to_move.insert(item);
                    stack.push((item.0 - 1, item.1));
                    stack.push((item.0 + dir.0, item.1 + dir.1));
                }
                '#' => {
                    valid = false;
                    break;
                }
                _ => unreachable!(),
            }
        }
        if valid {
            let mut old = Vec::with_capacity(to_move.len());
            for item in to_move {
                old.push((item, map[(item.0 + item.1 * width) as usize]));
                map[(item.0 + item.1 * width) as usize] = '.';
            }
            for (item, c) in old {
                map[((item.0 + dir.0) + (item.1 + dir.1) * width) as usize] = c;
            }
            pos = (pos.0 + dir.0, pos.1 + dir.1);
        }
    }

    let sum: usize = map
        .into_iter()
        .enumerate()
        .filter(|(_, c)| *c == '[')
        .map(|(i, _)| i % width as usize + i / width as usize * 100)
        .sum();
    println!("Result: {sum}");
}
