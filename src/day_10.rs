use aoc_runner_derive::aoc;
use itertools::Itertools;

const OPENING_CHARS: [char; 4] = ['(', '[', '{', '<'];
const CLOSING_CHARS: [char; 4] = [')', ']', '}', '>'];

#[aoc(day10, part1)]
pub fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut stack = Vec::new();

            let mismatch = line.chars().find(|character| match *character {
                character if OPENING_CHARS.contains(&character) => {
                    stack.push(character);
                    false
                }
                character => {
                    let opening_index = CLOSING_CHARS
                        .iter()
                        .position(|closer| *closer == character)
                        .unwrap();
                    OPENING_CHARS[opening_index] != stack.pop().unwrap()
                }
            });

            mismatch
                .map(|closer| match closer {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => unreachable!("Invalid closer char"),
                })
                .unwrap_or(0)
        })
        .sum()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &str) -> usize {
    let scores: Vec<_> = input
        .lines()
        .filter_map(|line| {
            let mut stack = Vec::new();

            let mismatch = line.chars().find(|character| match *character {
                character if OPENING_CHARS.contains(&character) => {
                    stack.push(character);
                    false
                }
                character => {
                    let opening_index = CLOSING_CHARS
                        .iter()
                        .position(|closer| *closer == character)
                        .unwrap();
                    OPENING_CHARS[opening_index] != stack.pop().unwrap()
                }
            });
            match mismatch {
                Some(_) => None,
                None => Some(stack.into_iter().rev().fold(0, |accumulator, opener| {
                    let value = match opener {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => unreachable!(),
                    };
                    accumulator * 5 + value
                })),
            }
        })
        .sorted_unstable()
        .collect();

    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(solve_part2(input), 288957);
    }
}
