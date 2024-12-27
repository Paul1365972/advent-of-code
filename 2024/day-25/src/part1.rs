use itertools::{iproduct, Itertools};

fn main() {
    let input = include_str!("../input.txt");

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for mut lines in input.lines().chunks(8).into_iter() {
        let is_lock = lines.next().unwrap().chars().all(|c| c == '#');
        let default = if is_lock { 0u8 } else { 5u8 };
        let mut values = [default; 5];
        for depth in 1..=5 {
            let line = lines.next().unwrap();
            for (pin, c) in line.char_indices() {
                if c == '#' && is_lock {
                    values[pin] = depth;
                } else if c == '.' && !is_lock {
                    values[pin] = 5 - depth;
                }
            }
        }
        lines.next().unwrap();
        lines.next();
        if is_lock {
            locks.push(values);
        } else {
            keys.push(values);
        }
    }

    let mut result: usize = 0;
    for (lock, key) in iproduct!(locks, keys) {
        let fits = lock
            .into_iter()
            .zip(key.into_iter())
            .all(|(lock, key)| lock + key <= 5);
        if fits {
            result += 1;
        }
    }
    println!("Result: {result}");
}
