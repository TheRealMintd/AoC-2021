use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(|position| position.parse().unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[isize]) -> isize {
    let upper_bound = input.iter().max().unwrap();

    (0..=*upper_bound)
        .map(|alignment| {
            input
                .iter()
                .map(|position| (*position - alignment).abs())
                .sum()
        })
        .min()
        .unwrap()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[isize]) -> isize {
    let upper_bound = input.iter().max().unwrap();

    (0..=*upper_bound)
        .map(|alignment| {
            input
                .iter()
                .map(|position| (1..=(*position - alignment).abs()).sum::<isize>())
                .sum()
        })
        .min()
        .unwrap()
}
