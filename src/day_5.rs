use std::{
    cmp::{max, min},
    collections::HashMap,
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

pub struct VentLine {
    end_one: Point,
    end_two: Point,
}

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn new(x: u16, y: u16) -> Self {
        Point { x, y }
    }
}

impl From<(u16, u16)> for Point {
    fn from((x, y): (u16, u16)) -> Self {
        Self::new(x, y)
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<VentLine> {
    input
        .lines()
        .map(|line| {
            let points: ((u16, u16), (u16, u16)) = line
                .split(" -> ")
                .map(|coordinates| {
                    coordinates
                        .split(',')
                        .map(|position| position.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap();

            VentLine {
                end_one: points.0.into(),
                end_two: points.1.into(),
            }
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[VentLine]) -> usize {
    input
        .iter()
        .filter(|vent| vent.end_one.x == vent.end_two.x || vent.end_one.y == vent.end_two.y)
        .flat_map::<Vec<Point>, _>(|vent| {
            if vent.end_one.x == vent.end_two.x {
                (min(vent.end_one.y, vent.end_two.y)..=max(vent.end_one.y, vent.end_two.y))
                    .map(|y| (vent.end_one.x, y).into())
                    .collect()
            } else {
                (min(vent.end_one.x, vent.end_two.x)..=max(vent.end_one.x, vent.end_two.x))
                    .map(|x| (x, vent.end_one.y).into())
                    .collect()
            }
        })
        .fold(HashMap::new(), |mut map: HashMap<_, u32>, point| {
            *map.entry(point).or_insert(0) += 1;
            map
        })
        .into_values()
        .filter(|count| *count > 1)
        .count()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[VentLine]) -> usize {
    input
        .iter()
        .flat_map::<Vec<Point>, _>(|vent| {
            if vent.end_one.x == vent.end_two.x {
                (min(vent.end_one.y, vent.end_two.y)..=max(vent.end_one.y, vent.end_two.y))
                    .map(|y| (vent.end_one.x, y).into())
                    .collect()
            } else if vent.end_one.y == vent.end_two.y {
                (min(vent.end_one.x, vent.end_two.x)..=max(vent.end_one.x, vent.end_two.x))
                    .map(|x| (x, vent.end_one.y).into())
                    .collect()
            } else {
                let x_range: Box<dyn Iterator<Item = u16>> = if vent.end_one.x > vent.end_two.x {
                    Box::new((vent.end_two.x..=vent.end_one.x).rev())
                } else {
                    Box::new(vent.end_one.x..=vent.end_two.x)
                };
                let y_range: Box<dyn Iterator<Item = u16>> = if vent.end_one.y > vent.end_two.y {
                    Box::new((vent.end_two.y..=vent.end_one.y).rev())
                } else {
                    Box::new(vent.end_one.y..=vent.end_two.y)
                };

                x_range.zip(y_range).map_into().collect()
            }
        })
        .fold(HashMap::new(), |mut map: HashMap<_, u32>, point| {
            *map.entry(point).or_insert(0) += 1;
            map
        })
        .into_values()
        .filter(|count| *count > 1)
        .count()
}
