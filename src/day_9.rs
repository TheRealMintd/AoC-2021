use std::ops::{Index, IndexMut};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Clone)]
pub struct Cave {
    spots: Vec<CaveSpot>,
    width: usize,
    height: usize,
}

#[derive(Clone, Copy, PartialEq)]
pub struct CaveSpot {
    height: u8,
    marked: bool,
}

impl Index<(usize, usize)> for Cave {
    type Output = CaveSpot;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.spots[y * self.width + x]
    }
}

impl IndexMut<(usize, usize)> for Cave {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.spots[y * self.width + x]
    }
}

impl PartialOrd for CaveSpot {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.height.partial_cmp(&other.height)
    }
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Cave {
    let mut width = 0;
    let spots = input
        .lines()
        .flat_map(|line| {
            width = line.len();
            line.chars().map(|height| CaveSpot {
                height: height.to_digit(10).unwrap() as u8,
                marked: false,
            })
        })
        .collect();

    Cave {
        spots,
        width,
        height: input.lines().count(),
    }
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &Cave) -> usize {
    let mut sum: usize = 0;
    for coordinates @ (x, y) in
        (0..input.height).flat_map(|y| (0..input.width).map(move |x| (x, y)))
    {
        let current_spot = input[coordinates];

        if (x == 0 || input[(x - 1, y)] > current_spot)
            && (x == input.width - 1 || input[(x + 1, y)] > current_spot)
            && (y == 0 || input[(x, y - 1)] > current_spot)
            && (y == input.height - 1 || input[(x, y + 1)] > current_spot)
        {
            sum += (current_spot.height + 1) as usize;
        }
    }

    sum
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Cave) -> usize {
    let mut owned_input = input.to_owned();

    (0..input.height)
        .flat_map(|y| (0..input.width).map(move |x| (x, y)))
        .map(|coordinates| traverse_basin(&mut owned_input, coordinates))
        .filter(|size| size > &0)
        .sorted()
        .rev()
        .take(3)
        .product()
}

fn traverse_basin(input: &mut Cave, coordinates @ (x, y): (usize, usize)) -> usize {
    let spot = &mut input[coordinates];
    if spot.marked || spot.height == 9 {
        return 0;
    }

    spot.marked = true;
    let mut count = 0;

    if x > 0 {
        count += traverse_basin(input, (x - 1, y));
    }
    if x < input.width - 1 {
        count += traverse_basin(input, (x + 1, y));
    }
    if y > 0 {
        count += traverse_basin(input, (x, y - 1));
    }
    if y < input.height - 1 {
        count += traverse_basin(input, (x, y + 1));
    }

    count + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";
        assert_eq!(solve_part1(&input_generator(input)), 15);
    }
}
