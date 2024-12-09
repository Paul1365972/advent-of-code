use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    let mut data = input
        .chars()
        .flat_map(|c| c.to_string().parse().ok())
        .enumerate()
        .flat_map(|(index, length)| {
            [if index % 2 == 0 {
                index as isize / 2
            } else {
                -1
            }]
            .repeat(length)
        })
        .collect_vec();

    let mut left = 0;
    let mut right = data.len() - 1;
    loop {
        while data[left] != -1 && left < right {
            left += 1;
        }
        while data[right] == -1 && left < right {
            right -= 1;
        }
        if left < right {
            data.swap(left, right);
        } else {
            break;
        }
    }

    let mut checksum = 0;
    for (index, id) in data.into_iter().take_while(|id| *id != -1).enumerate() {
        checksum += index * id as usize;
    }
    println!("Result: {checksum}");
}
