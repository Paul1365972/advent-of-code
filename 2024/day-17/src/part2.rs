// Program:
// 2,4,1,1,7,5,1,5,0,3,4,3,5,5,3,0

// Human readable version:
// b = a % 8
// b = b ^ 1
// c = a >> b
// b = b ^ 5
// a = a >> 3
// b = b ^ c
// out b % 8
// jmp a != 0

// Notes:
// a has to finish with the number 0
// Only the lowest 3 bits of a count in each iteration
// a also shifted down by 3 bits every iteration
// However a is used in c = a >> b, so we can't cheese the problem

fn main() {
    let program = [2, 4, 1, 1, 7, 5, 1, 5, 0, 3, 4, 3, 5, 5, 3, 0];
    let result = check(&program, 0).unwrap();
    println!("Result: {result}");
}

fn check(program: &[u8], initial_a: u64) -> Option<u64> {
    // println!("{program:?}, {initial_a}");
    if program.is_empty() {
        return Some(initial_a);
    }
    for i in 0..8 {
        let reg_a = (initial_a << 3) | i;

        let reg_b = reg_a & 7;
        let reg_b = reg_b ^ 1;
        let reg_c = reg_a >> reg_b;
        let reg_b = reg_b ^ 5;
        let reg_b = reg_b ^ reg_c;
        let out = reg_b & 7;
        if out == *program.last().unwrap() as u64 {
            let inner = check(&program[..program.len() - 1], reg_a);
            if inner.is_some() {
                return inner;
            }
        }
    }
    return None;
}
