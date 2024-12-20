use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");

    let mut lines = input.lines();
    let patterns = lines.next().unwrap().split(", ").collect_vec();
    let designs = lines.filter(|line| !line.is_empty()).collect_vec();

    let result = calculate(&patterns, &designs);

    println!("Result: {result}");
}

#[inline(never)]
fn calculate(patterns: &[&str], designs: &[&str]) -> u64 {
    let mut result = 0u64;
    for design in designs {
        let mut matched = vec![0u64; design.len() + 1];
        let mut buffer = vec![0u64; design.len() + 1];
        matched[0] = 1;

        loop {
            let mut progress = false;
            for (len, &possibilities) in matched.iter().enumerate() {
                if possibilities == 0 {
                    continue;
                }
                for pattern in patterns {
                    let new_len = len + pattern.len();
                    if new_len <= design.len() {
                        if *pattern == &design[len..new_len] {
                            progress = true;
                            buffer[new_len] += possibilities;
                            if new_len == design.len() {
                                result += possibilities;
                            }
                        }
                    }
                }
            }
            if !progress {
                break;
            }
            matched.fill(0);
            std::mem::swap(&mut matched, &mut buffer);
        }
    }
    return result;
}
