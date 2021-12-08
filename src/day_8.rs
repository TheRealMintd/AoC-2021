use aoc_runner_derive::{aoc, aoc_generator};
use bimap::BiHashMap;
use enumset::{EnumSet, EnumSetType};
use regex::Regex;

#[derive(Clone)]
pub struct NoteEntry {
    signals: Vec<EnumSet<State>>,
    digits: Vec<EnumSet<State>>,
}

impl NoteEntry {
    pub fn get_state<P>(&mut self, predicate: P) -> EnumSet<State>
    where
        P: Fn(&EnumSet<State>) -> bool,
    {
        let index = self.signals.iter().position(predicate).unwrap();
        self.signals.remove(index)
    }
}

#[derive(Debug, EnumSetType)]
pub enum State {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<NoteEntry> {
    let regex = Regex::new(r"(?-u)^(?P<signals>(?:\w+ ){10})\|(?P<digits>(?: \w+){4})$").unwrap();

    fn set_builder(accumulator: EnumSet<State>, character: char) -> EnumSet<State> {
        accumulator
            | match character {
                'a' => State::A,
                'b' => State::B,
                'c' => State::C,
                'd' => State::D,
                'e' => State::E,
                'f' => State::F,
                'g' => State::G,
                _ => unreachable!(),
            }
    }

    input
        .lines()
        .map(|line| {
            let cap = regex.captures(line).unwrap();
            NoteEntry {
                signals: cap
                    .name("signals")
                    .unwrap()
                    .as_str()
                    .split_ascii_whitespace()
                    .map(|signal| signal.chars().fold(EnumSet::new(), set_builder))
                    .collect(),
                digits: cap
                    .name("digits")
                    .unwrap()
                    .as_str()
                    .split_ascii_whitespace()
                    .map(|digit| digit.chars().fold(EnumSet::new(), set_builder))
                    .collect(),
            }
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[NoteEntry]) -> usize {
    input
        .iter()
        .map(|entry| {
            entry
                .digits
                .iter()
                .filter(|digit| [2, 3, 4, 7].contains(&digit.len()))
                .count()
        })
        .sum()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[NoteEntry]) -> usize {
    let eight: EnumSet<State> = EnumSet::all();

    input
        .iter()
        .cloned()
        .map(|mut entry| {
            let mut known_numbers = BiHashMap::with_capacity(10);
            known_numbers.insert(eight, 8);

            // add easily identifiable numbers
            for (index, length) in [2, 4, 3, 7].into_iter().enumerate() {
                const NUMBERS: [usize; 4] = [1, 4, 7, 8];
                known_numbers.insert(
                    entry.get_state(|state| state.len() == length),
                    NUMBERS[index],
                );
            }

            // find the rest
            known_numbers.insert(
                entry.get_state(|state| {
                    state.len() == 5 && state.is_superset(*known_numbers.get_by_right(&1).unwrap())
                }),
                3,
            );
            known_numbers.insert(
                entry.get_state(|state| {
                    state.len() == 5
                        && (*state ^ *known_numbers.get_by_right(&4).unwrap()).len() == 5
                }),
                2,
            );
            known_numbers.insert(entry.get_state(|state| state.len() == 5), 5);
            known_numbers.insert(
                entry.get_state(|state| {
                    *state
                        == (*known_numbers.get_by_right(&1).unwrap()
                            | *known_numbers.get_by_right(&5).unwrap())
                }),
                9,
            );
            known_numbers.insert(
                entry.get_state(|state| {
                    *state
                        == !(!*known_numbers.get_by_right(&5).unwrap()
                            & *known_numbers.get_by_right(&1).unwrap())
                }),
                6,
            );
            known_numbers.insert(entry.get_state(|_| true), 0);

            entry
                .digits
                .iter()
                .enumerate()
                .map(|(index, digit)| {
                    known_numbers.get_by_left(digit).unwrap() * (10usize.pow(3 - index as u32))
                })
                .sum::<usize>()
        })
        .sum()
}
