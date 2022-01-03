use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use ndarray::prelude::*;

#[derive(Debug)]
pub struct Manual {
    paper: Array2<bool>,
    instructions: Vec<Fold>,
}

#[derive(Debug)]
pub enum Fold {
    X(usize),
    Y(usize),
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Manual {
    let (paper, instructions) = input.trim().split("\n\n").collect_tuple().unwrap();

    let paper = {
        let cells = paper
            .lines()
            .map(|line| -> (usize, usize) {
                let (x, y) = line.split(',').collect_tuple().unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect_vec();

        let rows = cells.iter().map(|(x, _)| x).max().unwrap();
        let cols = cells.iter().map(|(_, y)| y).max().unwrap();

        let mut paper = Array2::from_elem((*cols + 1, *rows + 1), false);

        for (x, y) in cells {
            paper[(y, x)] = true;
        }

        paper
    };

    let instructions = instructions
        .lines()
        .map(|line| {
            let (orientation, amount) = line.split('=').collect_tuple().unwrap();
            let amount = amount.parse().unwrap();
            match orientation {
                "fold along y" => Fold::Y(amount),
                "fold along x" => Fold::X(amount),
                _ => unreachable!(),
            }
        })
        .collect_vec();

    Manual {
        paper,
        instructions,
    }
}

fn fold<'a>(paper: Array2<bool>, instrs: impl Iterator<Item = &'a Fold>) -> Array2<bool> {
    instrs.fold(paper, |paper, fold| {
        let (axis, idx) = match fold {
            Fold::Y(idx) => (Axis(0), *idx),
            Fold::X(idx) => (Axis(1), *idx),
        };

        let (a, almost_b) = paper.view().split_at(axis, idx);
        let (_, mut b) = almost_b.split_at(axis, 1);
        b.invert_axis(axis);

        a.to_owned() | b
    })
}

fn count_dots(paper: &Array2<bool>) -> usize {
    paper.iter().filter(|&&c| c).count()
}

#[aoc(day13, part1)]
pub fn part1(
    Manual {
        paper,
        instructions,
    }: &Manual,
) -> usize {
    let folded = fold(paper.clone(), instructions.iter().take(1));
    count_dots(&folded)
}

#[aoc(day13, part2)]
pub fn part2(
    Manual {
        paper,
        instructions,
    }: &Manual,
) -> usize {
    let folded = fold(paper.clone(), instructions.iter());
    count_dots(&folded)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 17);
    }

    #[test]
    fn solution1() {
        assert_eq!(
            part1(&input_generator(include_str!("../input/2021/day13.txt"))),
            610,
        );
    }

    #[test]
    fn solution2() {
        assert_eq!(
            part2(&input_generator(include_str!("../input/2021/day13.txt"))),
            95,
        );
    }
}
