use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;
use std::collections::HashSet;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn inc_at(grid: &mut [Vec<u8>], h: usize, w: usize) {
    if let Some(row) = grid.get_mut(h) {
        if let Some(energy) = row.get_mut(w) {
            *energy += 1;
        }
    }
}

fn step(mut grid: Vec<Vec<u8>>) -> (usize, Vec<Vec<u8>>) {
    let height = grid.len();
    let width = grid[0].len();

    for (h, w) in iproduct!(0..height, 0..width) {
        grid[h][w] += 1;
    }

    let mut flashed = HashSet::new();

    loop {
        let mut should_stop = true;

        for (h, w) in iproduct!(0..height, 0..width) {
            if !flashed.contains(&(h, w)) && grid[h][w] > 9 {
                flashed.insert((h, w));
                should_stop = false;

                inc_at(&mut grid, h.wrapping_sub(1), w.wrapping_sub(1));
                inc_at(&mut grid, h.wrapping_sub(1), w);
                inc_at(&mut grid, h.wrapping_sub(1), w + 1);
                inc_at(&mut grid, h, w.wrapping_sub(1));
                inc_at(&mut grid, h, w + 1);
                inc_at(&mut grid, h + 1, w.wrapping_sub(1));
                inc_at(&mut grid, h + 1, w);
                inc_at(&mut grid, h + 1, w + 1);
            }
        }

        if should_stop {
            break;
        }
    }

    for (h, w) in iproduct!(0..height, 0..width) {
        if grid[h][w] > 9 {
            grid[h][w] = 0;
        }
    }

    (flashed.len(), grid)
}

#[aoc(day11, part1)]
pub fn part1(input: &[Vec<u8>]) -> usize {
    (0..100)
        .fold((0usize, input.to_vec()), |(acc, grid), _| {
            let (inc, next_grid) = step(grid);
            (acc + inc, next_grid)
        })
        .0
}

#[aoc(day11, part2)]
pub fn part2(input: &[Vec<u8>]) -> usize {
    let mut i = 1;

    let octopusses = input.len() * input[0].len();
    let mut grid = input.to_vec();

    loop {
        let (flashes, next_grid) = step(grid);
        if flashes == octopusses {
            return i;
        }
        i += 1;
        grid = next_grid;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn step_() {
        let grid = input_generator(
            "11111
19991
19191
19991
11111",
        );
        let (flashes, grid) = step(grid);
        assert_eq!(flashes, 9);
        assert_eq!(
            grid,
            input_generator(
                "34543
40004
50005
40004
34543"
            )
        );

        let (flashes, grid) = step(grid);
        assert_eq!(flashes, 0);
        assert_eq!(
            grid,
            input_generator(
                "45654
51115
61116
51115
45654"
            )
        );
    }

    #[test]
    fn step_2() {
        let grid = input_generator(EXAMPLE);
        let (flashes, grid) = step(grid);
        assert_eq!(flashes, 0);
        assert_eq!(
            grid,
            input_generator(
                "6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637"
            )
        );

        let (flashes, grid) = step(grid);
        assert_eq!(flashes, 35);
        assert_eq!(
            grid,
            input_generator(
                "8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848"
            )
        );
    }

    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 1656);
    }

    #[test]
    fn solution1() {
        assert_eq!(
            part1(&input_generator(include_str!("../input/2021/day11.txt"))),
            1546,
        );
    }

    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(EXAMPLE)), 195);
    }

    #[test]
    fn solution2() {
        assert_eq!(
            part2(&input_generator(include_str!("../input/2021/day11.txt"))),
            471,
        );
    }
}
