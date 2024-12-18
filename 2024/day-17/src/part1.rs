use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");

    let mut lines = input.lines();
    let mut a = lines.next().unwrap()[12..].parse::<u64>().unwrap();
    let mut b = lines.next().unwrap()[12..].parse::<u64>().unwrap();
    let mut c = lines.next().unwrap()[12..].parse::<u64>().unwrap();
    lines.next().unwrap();
    let program = lines.next().unwrap()[9..]
        .split(',')
        .map(|str| str.parse::<u8>().unwrap())
        .collect_vec();

    let mut out = Vec::new();

    let mut ip = 0;
    loop {
        if ip + 2 > program.len() {
            break;
        }
        let opcode = program[ip];
        let operand_literal = program[ip + 1] as u64;
        let operand_combo = match operand_literal {
            0..=3 => operand_literal,
            4 => a,
            5 => b,
            6 => c,
            _ => unreachable!(),
        };
        match opcode {
            // adv
            0 => {
                a = if operand_combo < 64 {
                    a >> operand_combo
                } else {
                    0
                };
                ip += 2;
            }
            // bxl
            1 => {
                b = b ^ operand_literal;
                ip += 2;
            }
            // bst
            2 => {
                b = operand_combo % 8;
                ip += 2;
            }
            // jnz
            3 => {
                if a != 0 {
                    ip = operand_literal as usize;
                } else {
                    ip += 2;
                }
            }
            // bxc
            4 => {
                b = b ^ c;
                ip += 2;
            }
            // out
            5 => {
                out.push(operand_combo % 8);
                ip += 2;
            }
            // bdv
            6 => {
                b = if operand_combo < 64 {
                    a >> operand_combo
                } else {
                    0
                };
                ip += 2;
            }
            // cdv
            7 => {
                c = if operand_combo < 64 {
                    a >> operand_combo
                } else {
                    0
                };
                ip += 2;
            }
            _ => unreachable!(),
        }
    }

    println!("Result: {}", out.into_iter().join(","));
}
