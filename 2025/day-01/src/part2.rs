use std::fs;

use itertools::Itertools;

#[derive(Debug)]
enum Rotation {
    Left(u32),
    Right(u32),
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let rotations = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let direction = &line[0..=0];
            let value = line[1..].parse().unwrap();
            if direction == "L" {
                Rotation::Left(value)
            } else {
                Rotation::Right(value)
            }
        })
        .collect_vec();

    let max_dial_rotation = 100;
    let mut dial_rotation = 50;
    let mut hits = 0;
    for rotation in rotations {
        let rotation = match rotation {
            Rotation::Left(rotation) => -(rotation as i32),
            Rotation::Right(rotation) => rotation as i32,
        };

        let new = dial_rotation + rotation % max_dial_rotation;

        if dial_rotation != 0 && new <= 0 || new >= 100 {
            hits += 1;
        }
        hits += rotation.abs() / max_dial_rotation;

        dial_rotation = (new).rem_euclid(max_dial_rotation);
        println!("Rotation: {rotation}, New: {dial_rotation}, Hits: {hits}");
    }

    println!("Result: {}", hits);
}
