use itertools::Itertools;

fn main() {
    // Hope it is somewhere in the first 100k iterations
    const TIME: usize = 100_000;
    const WIDTH: usize = 101;
    const HEIGHT: usize = 103;

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

    let mut best_score = usize::MAX;
    let mut best_iteration = 0;
    for iteration in 1..=TIME {
        for robot in robots.iter_mut() {
            robot.pos.0 = (robot.pos.0 + robot.vel.0).rem_euclid(WIDTH as i32);
            robot.pos.1 = (robot.pos.1 + robot.vel.1).rem_euclid(HEIGHT as i32);
        }
        // Try to find the lowest "entropy", kind of inspired by run-length encoding, but not really
        // It does work though and successfully finds the easter egg, so all is well
        let mut map = [false; WIDTH * HEIGHT];
        for robot in robots.iter() {
            map[robot.pos.0 as usize + robot.pos.1 as usize * WIDTH] = true;
        }
        // Score: lower is better
        let mut score = 0;
        let mut state = false;
        for element in map {
            if state != element {
                state = element;
                score += 1;
            }
        }
        if score < best_score {
            best_score = score;
            best_iteration = iteration;
            for row in &map
                .into_iter()
                .map(|e| if e { 'O' } else { '.' })
                .chunks(WIDTH)
            {
                println!("{}", row.collect::<String>());
            }
            println!("Found new best iteration {iteration} with score {score}");
        }
    }

    println!("Result: {}", best_iteration);
}

struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}
