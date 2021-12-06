use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<u8> {
    input
        .split(',')
        .map(|timer| timer.parse().unwrap())
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[u8]) -> usize {
    solve(input, 80)
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[u8]) -> usize {
    solve(input, 256)
}

fn solve(input: &[u8], iterations: usize) -> usize {
    let mut fish_stages = VecDeque::from([0; 9]);
    input
        .iter()
        .for_each(|timer| fish_stages[(*timer) as usize] += 1);

    for _ in 0..iterations {
        let birthing_fish = fish_stages.pop_front().unwrap();
        fish_stages[6] += birthing_fish;
        fish_stages.push_back(birthing_fish);
    }

    fish_stages.into_iter().sum()
}
