use itertools::Itertools;
use rustc_hash::FxHashSet;

fn main() {
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
            let mut perimeter = 0;
            let identifier = get(&data, width, height, start_x, start_y).unwrap();
            let mut stack = Vec::from_iter([(start_x, start_y)]);
            while let Some(pos) = stack.pop() {
                if visited.contains(&pos) {
                    continue;
                }
                visited.insert(pos);
                area += 1;
                for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let neighbor = get(&data, width, height, pos.0 + dir.0, pos.1 + dir.1);
                    if neighbor == Some(identifier) {
                        stack.push((pos.0 + dir.0, pos.1 + dir.1));
                    } else {
                        perimeter += 1;
                    }
                }
            }
            println!("Found region '{identifier}' with area: {area} and perimeter: {perimeter}");
            cost += area * perimeter;
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
