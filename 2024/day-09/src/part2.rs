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

    let max_id = *data.iter().max().unwrap();

    for file_id in (0..=max_id).rev() {
        let mut file_pos = 0;
        while data[file_pos] != file_id {
            file_pos += 1;
        }
        let mut file_len = 0;
        while file_pos + file_len < data.len() && data[file_pos + file_len] == file_id {
            file_len += 1;
        }

        let mut space_pos = 0;
        while space_pos < file_pos {
            while space_pos < file_pos && data[space_pos] != -1 {
                space_pos += 1;
            }
            if space_pos >= file_pos {
                break;
            }
            let mut space_len = 0;
            while space_pos + space_len < data.len() && data[space_pos + space_len] == -1 {
                space_len += 1;
            }
            if space_len >= file_len {
                let (first, second) = data.split_at_mut(file_pos);
                first[space_pos..space_pos + file_len].swap_with_slice(&mut second[..file_len]);
                break;
            } else {
                space_pos += space_len;
            }
        }
    }

    let mut checksum = 0;
    for (index, id) in data.into_iter().enumerate().filter(|(_, id)| *id != -1) {
        checksum += index * id as usize;
    }
    println!("Result: {checksum}");
}
