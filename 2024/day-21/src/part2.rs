use itertools::{iproduct, Itertools};
use rustc_hash::{FxHashMap, FxHashSet};
use strum::VariantArray;

const DEPTH: usize = 2;

fn main() {
    let input = include_str!("../input.txt");

    let codes = input.lines().filter(|s| !s.is_empty()).collect_vec();

    let mut result = 0;
    for code in codes {
        // let length = todo!();
        // let numeric = code[0..=2].parse::<usize>().unwrap();
        // result += length * numeric;
    }

    println!("Result: {}", result);
}

fn compute_all_paths() {
    let mut lengths: FxHashMap<(usize, DirectionalKeypad, DirectionalKeypad), usize> =
        FxHashMap::default();
    for (start, target) in iproduct!(DirectionalKeypad::VARIANTS, DirectionalKeypad::VARIANTS) {}
}

fn compute_lengths() {
    let mut lengths: FxHashMap<(usize, DirectionalKeypad, DirectionalKeypad), usize> =
        FxHashMap::default();
    for depth in 0..DEPTH {
        for (start, target) in iproduct!(DirectionalKeypad::VARIANTS, DirectionalKeypad::VARIANTS) {
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, VariantArray)]
enum DirectionalKeypad {
    Up,
    Down,
    Left,
    Right,
    A,
}

impl DirectionalKeypad {
    fn apply(
        &self,
        input: DirectionalKeypad,
    ) -> (Option<DirectionalKeypad>, Option<DirectionalKeypad>) {
        if input == DirectionalKeypad::A {
            return (Some(*self), Some(*self));
        }
        let new_state = match (input, self) {
            (DirectionalKeypad::Right, DirectionalKeypad::Up) => Some(DirectionalKeypad::A),
            (DirectionalKeypad::Down, DirectionalKeypad::Up) => Some(DirectionalKeypad::Down),
            (DirectionalKeypad::Left, DirectionalKeypad::A) => Some(DirectionalKeypad::Up),
            (DirectionalKeypad::Down, DirectionalKeypad::A) => Some(DirectionalKeypad::Right),
            (DirectionalKeypad::Right, DirectionalKeypad::Left) => Some(DirectionalKeypad::Down),
            (DirectionalKeypad::Left, DirectionalKeypad::Down) => Some(DirectionalKeypad::Left),
            (DirectionalKeypad::Up, DirectionalKeypad::Down) => Some(DirectionalKeypad::Up),
            (DirectionalKeypad::Right, DirectionalKeypad::Down) => Some(DirectionalKeypad::Right),
            (DirectionalKeypad::Left, DirectionalKeypad::Right) => Some(DirectionalKeypad::Down),
            (DirectionalKeypad::Up, DirectionalKeypad::Right) => Some(DirectionalKeypad::A),
            _ => None,
        };
        return (new_state, None);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum NumericKeypad {
    Number0,
    Number1,
    Number2,
    Number3,
    Number4,
    Number5,
    Number6,
    Number7,
    Number8,
    Number9,
    A,
}
