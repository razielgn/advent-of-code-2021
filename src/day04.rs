use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashSet;

static MARKED: u8 = 255;

#[derive(Debug, Clone, Copy)]
struct Board {
    by_rows: [u8; 25],
    by_cols: [u8; 25],
}

fn transpose(rows: [u8; 25]) -> [u8; 25] {
    let mut cols = <[u8; 25]>::default();

    for i in 0..5 {
        for j in 0..5 {
            cols[j + i * 5] = rows[i + j * 5];
        }
    }

    cols
}

impl Board {
    fn from_rows(by_rows: [u8; 25]) -> Self {
        Board {
            by_rows,
            by_cols: transpose(by_rows),
        }
    }
}

#[derive(Debug)]
pub struct Game {
    drawn: Vec<u8>,
    boards: Vec<Board>,
}

impl Board {
    fn mark(&mut self, drawn: u8) {
        self.by_rows = self.by_rows.map(|n| if n == drawn { MARKED } else { n });
        self.by_cols = self.by_cols.map(|n| if n == drawn { MARKED } else { n });
    }

    fn has_won(&self) -> bool {
        self.by_rows
            .chunks(5)
            .any(|row| row.iter().all(|n| *n == MARKED))
            || self
                .by_cols
                .chunks(5)
                .any(|col| col.iter().all(|n| *n == MARKED))
    }

    fn sum_unmarked(&self) -> u32 {
        self.by_rows
            .iter()
            .filter_map(|n| {
                if *n == MARKED {
                    None
                } else {
                    Some(u32::from(*n))
                }
            })
            .sum()
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Game {
    let mut lines = input.lines();
    let drawn = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect_vec();

    let boards = lines
        .chunks(6)
        .into_iter()
        .map(|lines| {
            Board::from_rows(
                lines
                    .skip(1)
                    .flat_map(|line| line.split_whitespace().map(|s| s.parse().unwrap()))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            )
        })
        .collect_vec();

    Game { drawn, boards }
}

#[aoc(day4, part1)]
pub fn part1(input: &Game) -> u32 {
    let mut boards = input.boards.clone();

    for drawn in &input.drawn {
        for board in &mut boards {
            board.mark(*drawn);

            if board.has_won() {
                return board.sum_unmarked() * u32::from(*drawn);
            }
        }
    }

    unreachable!()
}

#[aoc(day4, part2)]
pub fn part2(input: &Game) -> u32 {
    let mut boards = input.boards.clone();

    let mut won = HashSet::new();
    let mut last_won_with_drawn = None;

    for drawn in &input.drawn {
        for (idx, board) in boards.iter_mut().enumerate() {
            if won.contains(&idx) {
                continue;
            }

            board.mark(*drawn);

            if board.has_won() {
                won.insert(idx);
                last_won_with_drawn = Some((*board, *drawn));
            }
        }
    }

    let (board, drawn) = last_won_with_drawn.unwrap();
    board.sum_unmarked() * u32::from(drawn)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 4_512);
    }

    #[test]
    fn solution1() {
        assert_eq!(
            part1(&input_generator(include_str!("../input/2021/day4.txt"))),
            27_027,
        );
    }

    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(EXAMPLE)), 1_924);
    }

    #[test]
    fn solution2() {
        assert_eq!(
            part2(&input_generator(include_str!("../input/2021/day4.txt"))),
            36_975,
        );
    }
}
