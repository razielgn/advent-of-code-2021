use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::{
    iter::{once, successors},
    ops::{Add, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Point {
    fn norm(&self) -> f64 {
        f64::from(self.x.pow(2) + self.y.pow(2)).sqrt()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    from: Point,
    to: Point,
}

impl Line {
    fn is_hor_or_vert(&self) -> bool {
        self.from.x == self.to.x || self.from.y == self.to.y
    }

    fn points(&self) -> impl Iterator<Item = Point> + '_ {
        let dist = self.from - self.to;
        let norm = dist.norm();
        let dir = Point {
            x: (f64::from(dist.x) / norm).round() as i32,
            y: (f64::from(dist.y) / norm).round() as i32,
        };

        successors(Some(self.to), move |point| Some(*point + dir))
            .take_while(|point| *point != self.from)
            .chain(once(self.from))
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split(" -> ").collect_tuple().unwrap();
            let (x1, y1) = a
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap();
            let (x2, y2) = b
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap();

            Line {
                from: Point { x: x1, y: y1 },
                to: Point { x: x2, y: y2 },
            }
        })
        .collect()
}

fn create_grid(input: &[Line]) -> Vec<Vec<u8>> {
    let width = input
        .iter()
        .flat_map(|Line { from, to }| [from.x, to.x])
        .max()
        .unwrap()
        + 1;
    let height = input
        .iter()
        .flat_map(|Line { from, to }| [from.y, to.y])
        .max()
        .unwrap()
        + 1;

    vec![vec![0; width as usize]; height as usize]
}

#[aoc(day5, part1)]
pub fn part1(input: &[Line]) -> usize {
    let grid = input.iter().filter(|line| line.is_hor_or_vert()).fold(
        create_grid(input),
        |mut grid, line| {
            for point in line.points() {
                grid[point.y as usize][point.x as usize] += 1;
            }

            grid
        },
    );

    grid.into_iter()
        .flat_map(|rows| rows.into_iter())
        .filter(|c| *c >= 2)
        .count()
}

#[aoc(day5, part2)]
pub fn part2(input: &[Line]) -> usize {
    let grid = input.iter().fold(create_grid(input), |mut grid, line| {
        for point in line.points() {
            grid[point.y as usize][point.x as usize] += 1;
        }

        grid
    });

    grid.into_iter()
        .flat_map(|rows| rows.into_iter())
        .filter(|c| *c >= 2)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 5);
    }

    #[test]
    fn solution1() {
        assert_eq!(
            part1(&input_generator(include_str!("../input/2021/day5.txt"))),
            7_297,
        );
    }

    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(EXAMPLE)), 12);
    }

    #[test]
    fn line_points() {
        assert_eq!(
            vec![
                Point { x: 1, y: 3 },
                Point { x: 1, y: 2 },
                Point { x: 1, y: 1 }
            ],
            Line {
                from: Point { x: 1, y: 1 },
                to: Point { x: 1, y: 3 }
            }
            .points()
            .collect_vec()
        );

        assert_eq!(
            vec![
                Point { x: 3, y: 3 },
                Point { x: 2, y: 2 },
                Point { x: 1, y: 1 }
            ],
            Line {
                from: Point { x: 1, y: 1 },
                to: Point { x: 3, y: 3 }
            }
            .points()
            .collect_vec()
        );
    }

    #[test]
    fn solution2() {
        assert_eq!(
            part2(&input_generator(include_str!("../input/2021/day5.txt"))),
            21_038,
        );
    }
}
