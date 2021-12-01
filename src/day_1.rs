use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|number| number.parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    input
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    let sums: Vec<usize> = input.windows(3).map(|window| window.iter().sum()).collect();
    sums.windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}
