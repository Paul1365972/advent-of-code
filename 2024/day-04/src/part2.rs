use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    let data = input.chars().filter(|c| c.is_alphabetic()).collect_vec();
    let height = input.lines().filter(|s| !s.is_empty()).count();
    let width = data.len() / height;
    assert_eq!(width * height, data.len());

    let field = Field {
        data,
        width,
        height,
    };

    let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    let mut count = 0;
    for i in 1..width as isize {
        for j in 1..height as isize {
            for dir in directions {
                if check_valid(&field, (i, j), dir) {
                    count += 1;
                }
            }
        }
    }
    println!("Count: {count}");
}

fn check_valid(field: &Field, pos: (isize, isize), dir: (isize, isize)) -> bool {
    return field.get(pos.0 + dir.0, pos.1 + dir.1) == 'M'
        && field.get(pos.0 + dir.1, pos.1 - dir.0) == 'M'
        && field.get(pos.0, pos.1) == 'A'
        && field.get(pos.0 - dir.0, pos.1 - dir.1) == 'S'
        && field.get(pos.0 - dir.1, pos.1 + dir.0) == 'S';
}

struct Field {
    data: Vec<char>,
    width: usize,
    height: usize,
}

impl Field {
    fn get(&self, x: isize, y: isize) -> char {
        return if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            self.data[x as usize + y as usize * self.width]
        } else {
            ' '
        };
    }
}
