use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &[i32]) -> i32 {
    let max = input.iter().max().unwrap();

    (0..*max)
        .map(|i| input.iter().map(|pos| (i - pos).abs()).sum())
        .min()
        .unwrap()
}

#[aoc(day7, part2)]
pub fn part2(input: &[i32]) -> i32 {
    let max = input.iter().max().unwrap();

    (0..*max)
        .map(|i| {
            input
                .iter()
                .map(|pos| {
                    let diff = (i - pos).abs();
                    ((diff * diff) + diff) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 37);
    }

    #[test]
    fn solution1() {
        assert_eq!(
            part1(&input_generator(include_str!("../input/2021/day7.txt"))),
            359_648,
        );
    }

    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(EXAMPLE)), 168);
    }

    #[test]
    fn solution2() {
        assert_eq!(
            part2(&input_generator(include_str!("../input/2021/day7.txt"))),
            100_727_924,
        );
    }
}
