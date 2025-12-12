use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();

    let tiles: Vec<(i64, i64)> = input
        .trim()
        .lines()
        .map(|line| {
            line.split(",")
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    let (a, b) = (0..tiles.len())
        .flat_map(|a| (0..a).map(move |b| (a, b)))
        .max_by_key(|(a, b)| size(tiles[*a], tiles[*b]))
        .unwrap();

    let result = size(tiles[a], tiles[b]);

    println!("Result: {}", result);
}

fn size(a: (i64, i64), b: (i64, i64)) -> i64 {
    return ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1);
}
