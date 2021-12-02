use aoc_runner_derive::{aoc, aoc_generator};

pub enum Move {
    Forward(usize),
    Down(usize),
    Up(usize),
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split_ascii_whitespace();
            let direction = split.next().unwrap();
            let amplitude: usize = split.next().unwrap().parse().unwrap();

            match direction {
                "forward" => Move::Forward(amplitude),
                "down" => Move::Down(amplitude),
                "up" => Move::Up(amplitude),
                _ => unreachable!("Invalid move"),
            }
        })
        .collect()
}

#[derive(Default)]
struct Position {
    horizontal: usize,
    depth: usize,
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Move]) -> usize {
    let final_position: Position =
        input
            .iter()
            .fold(Default::default(), |mut accumulator, one_move| {
                match one_move {
                    Move::Forward(amplitude) => accumulator.horizontal += amplitude,
                    Move::Down(amplitude) => accumulator.depth += amplitude,
                    Move::Up(amplitude) => accumulator.depth -= amplitude,
                };
                accumulator
            });

    final_position.horizontal * final_position.depth
}

#[derive(Default)]
struct ComplicatedPosition {
    aim: usize,
    horizontal: usize,
    depth: usize,
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Move]) -> usize {
    let final_position: ComplicatedPosition =
        input
            .iter()
            .fold(Default::default(), |mut accumulator, one_move| {
                match one_move {
                    Move::Forward(amplitude) => {
                        accumulator.horizontal += amplitude;
                        accumulator.depth += accumulator.aim * amplitude;
                    }
                    Move::Down(amplitude) => accumulator.aim += amplitude,
                    Move::Up(amplitude) => accumulator.aim -= amplitude,
                };
                accumulator
            });

    final_position.horizontal * final_position.depth
}
