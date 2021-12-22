use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;
use ndarray::prelude::*;

type Grid = Array2<u8>;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Grid {
    let height = input.trim().lines().count();
    let width = input.trim().lines().next().unwrap().chars().count();

    Array::from_iter(
        input
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u8)),
    )
    .into_shape((height, width))
    .unwrap()
}

fn step(mut grid: Grid) -> (usize, Grid) {
    fn inc(energy: &mut u8) {
        *energy += 1;
    }

    grid += 1;

    let mut flashed = Array2::from_elem((grid.ncols(), grid.nrows()), false);

    loop {
        let mut should_stop = true;

        for (h, w) in iproduct!(0..grid.ncols(), 0..grid.nrows()) {
            if grid[(h, w)] > 9 && !flashed[(h, w)] {
                flashed[(h, w)] = true;
                should_stop = false;

                grid.get_mut((h.wrapping_sub(1), w.wrapping_sub(1)))
                    .map(inc);
                grid.get_mut((h.wrapping_sub(1), w)).map(inc);
                grid.get_mut((h.wrapping_sub(1), w + 1)).map(inc);
                grid.get_mut((h, w.wrapping_sub(1))).map(inc);
                grid.get_mut((h, w + 1)).map(inc);
                grid.get_mut((h + 1, w.wrapping_sub(1))).map(inc);
                grid.get_mut((h + 1, w)).map(inc);
                grid.get_mut((h + 1, w + 1)).map(inc);
            }
        }

        if should_stop {
            break;
        }
    }

    grid.map_inplace(|energy| {
        if *energy > 9 {
            *energy = 0
        }
    });

    (flashed.iter().filter(|&&flashed| flashed).count(), grid)
}

#[aoc(day11, part1)]
pub fn part1(grid: &Grid) -> usize {
    (0..100)
        .fold((0usize, grid.clone()), |(acc, grid), _| {
            let (inc, next_grid) = step(grid);
            (acc + inc, next_grid)
        })
        .0
}

#[aoc(day11, part2)]
pub fn part2(grid: &Grid) -> usize {
    let mut i = 1;

    let octopusses = grid.ncols() * grid.nrows();
    let mut grid = grid.clone();

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
