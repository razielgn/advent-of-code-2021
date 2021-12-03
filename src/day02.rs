use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::str::FromStr;

pub enum Movement {
    Forward,
    Down,
    Up,
}

impl FromStr for Movement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Self::Forward),
            "down" => Ok(Self::Down),
            "up" => Ok(Self::Up),
            _ => Err(()),
        }
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(Movement, u32)> {
    input
        .lines()
        .map(|l| {
            let (movement, amount) = l.splitn(2, ' ').collect_tuple().unwrap();
            (movement.parse().unwrap(), amount.parse().unwrap())
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[(Movement, u32)]) -> u32 {
    let pos = input
        .iter()
        .fold((0, 0), |(hor, depth), (mov, amount)| match mov {
            Movement::Forward => (hor + amount, depth),
            Movement::Down => (hor, depth + amount),
            Movement::Up => (hor, depth - amount),
        });

    pos.0 * pos.1
}

#[aoc(day2, part2)]
pub fn part2(input: &[(Movement, u32)]) -> u32 {
    let pos = input
        .iter()
        .fold((0, 0, 0), |(hor, depth, aim), (mov, amount)| match mov {
            Movement::Down => (hor, depth, aim + amount),
            Movement::Up => (hor, depth, aim - amount),
            Movement::Forward => (hor + amount, depth + (aim * amount), aim),
        });

    pos.0 * pos.1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 150);
    }

    #[test]
    fn solution1() {
        assert_eq!(
            part1(&input_generator(include_str!("../input/2021/day2.txt"))),
            2_039_912
        );
    }

    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(EXAMPLE)), 900);
    }

    #[test]
    fn solution2() {
        assert_eq!(
            part2(&input_generator(include_str!("../input/2021/day2.txt"))),
            1_942_068_080,
        );
    }
}
