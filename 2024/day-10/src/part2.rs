use itertools::Itertools;

const DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn main() {
    let input = include_str!("../input.txt");

    let data = input.chars().filter(|c| !c.is_whitespace()).map(|c| c.to_string().parse::<u8>().unwrap()).collect_vec();
    let height = input.lines().filter(|s| !s.is_empty()).count() as isize;
    let width = data.len() as isize / height;

    let mut sum = 0;
    for start_y in 0..height {
        for start_x in 0..width {
            if get(&data, width, height, start_x, start_y) == Some(0) {
                let mut stack = Vec::from_iter([(start_x, start_y)]);
                let mut found = Vec::new();
                while let Some(pos) = stack.pop() {
                    let value = get(&data, width, height, pos.0, pos.1).unwrap();
                    if value == 9 {
                        found.push(pos);
                    } else {
                        for dir in DIRS {
                            let other_pos = (pos.0 + dir.0, pos.1 + dir.1);
                            let other_value = get(&data, width, height, other_pos.0, other_pos.1);
                            if let Some(other_data) = other_value {
                                if value + 1 == other_data {
                                    stack.push(other_pos);
                                }
                            }
                        }
                    }
                }
                sum += found.len();
            }
        }
    }

    println!("Result: {sum}");
}

fn get(data: &[u8], width: isize, height: isize, x: isize, y: isize) -> Option<u8> {
    return if x >= 0 && x < width && y >= 0 && y < height {
        Some(data[x as usize + y as usize * width as usize])
    } else {
        None
    };
}
