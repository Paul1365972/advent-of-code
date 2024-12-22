use std::collections::BinaryHeap;

use itertools::Itertools;
use rustc_hash::FxHashMap;

fn main() {
    let input = include_str!("../input.txt");

    let codes = input.lines().filter(|s| !s.is_empty()).collect_vec();

    let mut result = 0;
    for code in codes {
        let length = shortest_path(code);
        let numeric = code[0..=2].parse::<usize>().unwrap();
        result += length * numeric;
    }

    println!("Result: {}", result);
}

fn shortest_path(code: &str) -> usize {
    let target = NumericKeypad::from_code(code);

    let mut best = FxHashMap::default();
    let mut heap = BinaryHeap::new();
    heap.push((0, State::new()));
    while let Some((score, state)) = heap.pop() {
        if score < *best.get(&state).unwrap_or(&isize::MIN) {
            continue;
        }
        for input in [
            DirectionalKeypad::A,
            DirectionalKeypad::Up,
            DirectionalKeypad::Down,
            DirectionalKeypad::Left,
            DirectionalKeypad::Right,
        ] {
            let state = state.apply(input, target);
            let Some(state) = state else {
                continue;
            };
            let score = score - 1;
            let best_score = best.entry(state).or_insert(isize::MIN);
            if *best_score < score {
                *best_score = score;
                heap.push((score, state));
            }
        }
    }
    return -best
        .iter()
        .filter(|(state, _)| state.out[3].is_some())
        .map(|(_, score)| *score)
        .max()
        .unwrap() as usize;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    pad2: DirectionalKeypad,
    pad3: DirectionalKeypad,
    pad4: NumericKeypad,
    out: [Option<NumericKeypad>; 4],
}

impl State {
    fn new() -> Self {
        Self {
            pad2: DirectionalKeypad::A,
            pad3: DirectionalKeypad::A,
            pad4: NumericKeypad::A,
            out: [None; 4],
        }
    }

    fn apply(&self, input: DirectionalKeypad, reference_code: [NumericKeypad; 4]) -> Option<State> {
        let (pad2, out) = self.pad2.apply(input);
        let pad2 = pad2?;
        let Some(input) = out else {
            return Some(State { pad2, ..*self });
        };

        let (pad3, out) = self.pad3.apply(input);
        let pad3 = pad3?;
        let Some(input) = out else {
            return Some(State {
                pad2,
                pad3,
                ..*self
            });
        };

        let (pad4, out) = self.pad4.apply(input);
        let pad4 = pad4?;
        let Some(input) = out else {
            return Some(State {
                pad2,
                pad3,
                pad4,
                ..*self
            });
        };
        let next_out_index = self.out.iter().position(|x| x.is_none())?;
        if reference_code[next_out_index] != input {
            return None;
        }
        let mut new_out = self.out.clone();
        new_out[next_out_index] = Some(input);
        return Some(State {
            pad2,
            pad3,
            pad4,
            out: new_out,
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl NumericKeypad {
    fn apply(&self, input: DirectionalKeypad) -> (Option<NumericKeypad>, Option<NumericKeypad>) {
        if input == DirectionalKeypad::A {
            return (Some(*self), Some(*self));
        }
        let new_state = match (input, self) {
            (DirectionalKeypad::Up, NumericKeypad::Number0) => Some(NumericKeypad::Number2),
            (DirectionalKeypad::Up, NumericKeypad::Number1) => Some(NumericKeypad::Number4),
            (DirectionalKeypad::Up, NumericKeypad::Number2) => Some(NumericKeypad::Number5),
            (DirectionalKeypad::Up, NumericKeypad::Number3) => Some(NumericKeypad::Number6),
            (DirectionalKeypad::Up, NumericKeypad::Number4) => Some(NumericKeypad::Number7),
            (DirectionalKeypad::Up, NumericKeypad::Number5) => Some(NumericKeypad::Number8),
            (DirectionalKeypad::Up, NumericKeypad::Number6) => Some(NumericKeypad::Number9),
            (DirectionalKeypad::Up, NumericKeypad::A) => Some(NumericKeypad::Number3),
            (DirectionalKeypad::Down, NumericKeypad::Number2) => Some(NumericKeypad::Number0),
            (DirectionalKeypad::Down, NumericKeypad::Number4) => Some(NumericKeypad::Number1),
            (DirectionalKeypad::Down, NumericKeypad::Number5) => Some(NumericKeypad::Number2),
            (DirectionalKeypad::Down, NumericKeypad::Number6) => Some(NumericKeypad::Number3),
            (DirectionalKeypad::Down, NumericKeypad::Number7) => Some(NumericKeypad::Number4),
            (DirectionalKeypad::Down, NumericKeypad::Number8) => Some(NumericKeypad::Number5),
            (DirectionalKeypad::Down, NumericKeypad::Number9) => Some(NumericKeypad::Number6),
            (DirectionalKeypad::Down, NumericKeypad::Number3) => Some(NumericKeypad::A),
            (DirectionalKeypad::Left, NumericKeypad::Number2) => Some(NumericKeypad::Number1),
            (DirectionalKeypad::Left, NumericKeypad::Number3) => Some(NumericKeypad::Number2),
            (DirectionalKeypad::Left, NumericKeypad::Number5) => Some(NumericKeypad::Number4),
            (DirectionalKeypad::Left, NumericKeypad::Number6) => Some(NumericKeypad::Number5),
            (DirectionalKeypad::Left, NumericKeypad::Number8) => Some(NumericKeypad::Number7),
            (DirectionalKeypad::Left, NumericKeypad::Number9) => Some(NumericKeypad::Number8),
            (DirectionalKeypad::Left, NumericKeypad::A) => Some(NumericKeypad::Number0),
            (DirectionalKeypad::Right, NumericKeypad::Number1) => Some(NumericKeypad::Number2),
            (DirectionalKeypad::Right, NumericKeypad::Number2) => Some(NumericKeypad::Number3),
            (DirectionalKeypad::Right, NumericKeypad::Number4) => Some(NumericKeypad::Number5),
            (DirectionalKeypad::Right, NumericKeypad::Number5) => Some(NumericKeypad::Number6),
            (DirectionalKeypad::Right, NumericKeypad::Number7) => Some(NumericKeypad::Number8),
            (DirectionalKeypad::Right, NumericKeypad::Number8) => Some(NumericKeypad::Number9),
            (DirectionalKeypad::Right, NumericKeypad::Number0) => Some(NumericKeypad::A),
            _ => None,
        };
        return (new_state, None);
    }

    fn from_code(code: &str) -> [NumericKeypad; 4] {
        code.chars()
            .map(|char| match char {
                '0' => NumericKeypad::Number0,
                '1' => NumericKeypad::Number1,
                '2' => NumericKeypad::Number2,
                '3' => NumericKeypad::Number3,
                '4' => NumericKeypad::Number4,
                '5' => NumericKeypad::Number5,
                '6' => NumericKeypad::Number6,
                '7' => NumericKeypad::Number7,
                '8' => NumericKeypad::Number8,
                '9' => NumericKeypad::Number9,
                'A' => NumericKeypad::A,
                _ => unreachable!(),
            })
            .collect_vec()
            .try_into()
            .unwrap()
    }
}
