use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut occurence_count = [0; 12];
    input.lines().for_each(|line| {
        line.chars().enumerate().for_each(|(index, char)| {
            if char == '0' {
                occurence_count[index] -= 1;
            } else {
                occurence_count[index] += 1;
            }
        })
    });

    let gamma: usize = occurence_count
        .iter()
        .enumerate()
        .map(|(index, bit)| if bit > &0 { 1 << (11 - index) } else { 0 })
        .sum();
    let epsilon = !gamma & 0b111111111111;

    gamma * epsilon
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut oxygen_pool: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let mut scrubber_pool: Vec<_> = oxygen_pool.clone();

    for i in 0..12 {
        let occurence_count: i32 = oxygen_pool
            .iter()
            .map(|string| if string[i] == b'0' { -1 } else { 1 })
            .sum();
        oxygen_pool = oxygen_pool
            .into_iter()
            .filter(|string| {
                if occurence_count >= 0 {
                    string[i] == b'1'
                } else {
                    string[i] == b'0'
                }
            })
            .collect();
    }

    for i in 0..12 {
        let occurence_count: i32 = scrubber_pool
            .iter()
            .map(|string| if string[i] == b'0' { -1 } else { 1 })
            .sum();
        scrubber_pool = scrubber_pool
            .into_iter()
            .filter(|string| {
                if occurence_count < 0 {
                    string[i] == b'1'
                } else {
                    string[i] == b'0'
                }
            })
            .collect();

        if scrubber_pool.len() == 1 {
            break;
        }
    }

    let oxygen_rating =
        usize::from_str_radix(std::str::from_utf8(oxygen_pool[0]).unwrap(), 2).unwrap();
    let scrubber_rating =
        usize::from_str_radix(std::str::from_utf8(scrubber_pool[0]).unwrap(), 2).unwrap();

    oxygen_rating * scrubber_rating
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part2() {
        assert_eq!(
            solve_part2(
                &[
                    "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100",
                    "10000", "11001", "00010", "01010",
                ]
                .join("\n"),
            ),
            230
        );
    }
}
