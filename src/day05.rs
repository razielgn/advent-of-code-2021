use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
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
        let dist = (self.from.x - self.to.x, self.from.y - self.to.y);
        let norm = f64::from(dist.0.pow(2) + dist.1.pow(2)).sqrt();
        let dir = (
            (f64::from(dist.0) / norm) as i32,
            (f64::from(dist.1) / norm) as i32,
        );

        std::iter::successors(Some(self.to), move |Point { x, y }| {
            Some(Point {
                x: x + dir.0,
                y: y + dir.1,
            })
        })
        .take_while(|point| *point != self.from)
        .chain(std::iter::once(self.from))
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

#[aoc(day5, part1)]
pub fn part1(input: &[Line]) -> usize {
    let width = input
        .iter()
        .map(|Line { from, to }| [from.x, to.x])
        .flatten()
        .max()
        .unwrap()
        + 1;
    let height = input
        .iter()
        .map(|Line { from, to }| [from.y, to.y])
        .flatten()
        .max()
        .unwrap()
        + 1;

    let grid = input.iter().filter(|line| line.is_hor_or_vert()).fold(
        vec![vec![0; width as usize]; height as usize],
        |mut grid, line| {
            for point in line.points() {
                grid[point.y as usize][point.x as usize] += 1;
            }

            grid
        },
    );

    grid.into_iter()
        .map(|rows| rows.into_iter())
        .flatten()
        .filter(|c| *c >= 2)
        .count()
}

#[aoc(day5, part2)]
pub fn part2(_input: &[Line]) -> u16 {
    todo!()
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

    // #[test]
    // fn example2() {
    //     assert_eq!(part2(&input_generator(EXAMPLE)), 1_924);
    // }

    // #[test]
    // fn solution2() {
    //     assert_eq!(
    //         part2(&input_generator(include_str!("../input/2021/day5.txt"))),
    //         36_975,
    //     );
    // }
}
