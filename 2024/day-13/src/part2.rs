use regex::{Captures, Regex};

fn main() {
    let input = include_str!("../input.txt");

    let mut lines = input.lines();
    let mut machines = Vec::new();
    while let Some(machine) = next_machine(&mut lines) {
        machines.push(machine);
    }

    for machine in machines.iter_mut() {
        machine.goal.0 += 10_000_000_000_000;
        machine.goal.1 += 10_000_000_000_000;
    }

    let mut total_cost = 0;
    for machine in machines.iter() {
        let determinant = 1.0 / (machine.a.0 * machine.b.1 - machine.b.0 * machine.a.1) as f64;
        let a = (determinant
            * (machine.goal.0 * machine.b.1 + machine.goal.1 * -machine.b.0) as f64)
            .round() as i64;
        let b = (determinant
            * (machine.goal.0 * -machine.a.1 + machine.goal.1 * machine.a.0) as f64)
            .round() as i64;

        if a * machine.a.0 + b * machine.b.0 == machine.goal.0
            && a * machine.a.1 + b * machine.b.1 == machine.goal.1
        {
            let cost = 3 * a + 1 * b;
            total_cost += cost;
        }
    }

    println!("Result: {total_cost}");
}

struct ClawMachine {
    a: (i64, i64),
    b: (i64, i64),
    goal: (i64, i64),
}

fn next_machine<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Option<ClawMachine> {
    let coord_regex = Regex::new(r"^.*?: X=?([+-]?\d+), Y=?([+-]?\d+)$").unwrap();

    let a = coord_regex.captures(iter.next()?).unwrap();
    let b = coord_regex.captures(iter.next().unwrap()).unwrap();
    let goal = coord_regex.captures(iter.next().unwrap()).unwrap();
    iter.next();

    let extract =
        |capture: &Captures<'_>, index| capture.get(index).unwrap().as_str().parse().unwrap();

    Some(ClawMachine {
        a: (extract(&a, 1), extract(&a, 2)),
        b: (extract(&b, 1), extract(&b, 2)),
        goal: (extract(&goal, 1), extract(&goal, 2)),
    })
}
