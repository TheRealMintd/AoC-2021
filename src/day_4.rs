use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone)]
pub struct Bingo {
    numbers: Vec<usize>,
    number_position: usize,
    boards: Vec<Board>,
}

#[derive(Clone)]
struct Board {
    board: Vec<BingoNumber>,
}

#[derive(Copy, Clone)]
struct BingoNumber {
    number: usize,
    marked: bool,
}

impl Bingo {
    // disable false positive lint
    #[allow(clippy::needless_collect)]
    fn draw_number(&mut self) -> Option<(Board, usize)> {
        let number = self.numbers[self.number_position];
        self.number_position += 1;
        let changed_boards: Vec<_> = self
            .boards
            .iter_mut()
            .enumerate()
            .filter_map(|(index, board)| {
                board
                    .mark(number)
                    .map(|changed_position| (index, changed_position))
            })
            .collect();

        let winning_board = changed_boards
            .into_iter()
            .rev()
            .filter_map(|(index, changed_position)| {
                if self.boards[index].check(changed_position) {
                    Some(self.boards.remove(index))
                } else {
                    None
                }
            })
            .last();

        winning_board.map(|board| (board, number))
    }
}

impl Board {
    fn check(&self, changed_position: usize) -> bool {
        let row_start = changed_position / 5 * 5;
        let row_win = (0..5).all(|offset| self.board[row_start + offset].marked);

        row_win || {
            let column_start = changed_position % 5;
            (0..5).all(|offset| self.board[column_start + 5 * offset].marked)
        }
    }

    fn mark(&mut self, drawn_number: usize) -> Option<usize> {
        self.board
            .iter_mut()
            .enumerate()
            .find(|(_, BingoNumber { number, .. })| drawn_number == *number)
            .map(|(index, bingo_number)| {
                bingo_number.marked = true;
                index
            })
    }
}

impl BingoNumber {
    fn new(number: usize) -> Self {
        BingoNumber {
            number,
            marked: false,
        }
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Bingo {
    let mut sections = input.split("\n\n");
    let numbers = sections
        .next()
        .unwrap()
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect();

    let boards: Vec<Board> = sections
        .map(|board| Board {
            board: board
                .lines()
                .map(|row| {
                    row.split_ascii_whitespace()
                        .map(|number| BingoNumber::new(number.parse().unwrap()))
                })
                .flatten()
                .collect(),
        })
        .collect();

    Bingo {
        numbers,
        number_position: 0,
        boards,
    }
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Bingo) -> usize {
    let mut input = input.clone();
    loop {
        let result = input.draw_number();
        if let Some((board, winning_number)) = result {
            return board
                .board
                .iter()
                .filter(|BingoNumber { marked, .. }| !marked)
                .map(|BingoNumber { number, .. }| number)
                .sum::<usize>()
                * winning_number;
        }
    }
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Bingo) -> usize {
    let mut input = input.clone();
    loop {
        let result = input.draw_number();
        if let Some((board, winning_number)) = result {
            if input.boards.is_empty() {
                return board
                    .board
                    .iter()
                    .filter(|BingoNumber { marked, .. }| !marked)
                    .map(|BingoNumber { number, .. }| number)
                    .sum::<usize>()
                    * winning_number;
            }
        }
    }
}
