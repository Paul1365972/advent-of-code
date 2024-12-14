use itertools::Itertools;

fn main() {
    let time = 100;
    let width = 101;
    let height = 103;
    let input = include_str!("../input.txt");

    let mut robots = input
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(' ').unwrap();
            let parse = |s: &str| {
                let (x, y) = s.split_once(',').unwrap();
                return (x.parse().unwrap(), y.parse().unwrap());
            };
            Robot {
                pos: parse(&p[2..]),
                vel: parse(&v[2..]),
            }
        })
        .collect_vec();

    for robot in robots.iter_mut() {
        robot.pos.0 = (robot.pos.0 + robot.vel.0 * time).rem_euclid(width);
        robot.pos.1 = (robot.pos.1 + robot.vel.1 * time).rem_euclid(height);
    }

    let q1 = robots
        .iter()
        .filter(|r| r.pos.0 > width / 2 && r.pos.1 > height / 2)
        .count();
    let q2 = robots
        .iter()
        .filter(|r| r.pos.0 > width / 2 && r.pos.1 < height / 2)
        .count();
    let q3 = robots
        .iter()
        .filter(|r| r.pos.0 < width / 2 && r.pos.1 > height / 2)
        .count();
    let q4 = robots
        .iter()
        .filter(|r| r.pos.0 < width / 2 && r.pos.1 < height / 2)
        .count();

    let safety_factor: usize = [q1, q2, q3, q4].iter().product();

    println!("Result: {safety_factor}");
}

struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}
