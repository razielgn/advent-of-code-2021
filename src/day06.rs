use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn go(input: &[usize], days: usize) -> usize {
    let mut counts = [0; 9];
    for (n, count) in counts.iter_mut().enumerate() {
        *count = input.iter().filter(|&days| *days == n).count();
    }

    for _ in 0..days {
        let mut next = [0; 9];
        next[..8].copy_from_slice(&counts[1..]);

        // Those in 0 (in counts[0]) are turned into 6.
        next[6] += counts[0];
        // Those in 0 become 8.
        next[8] = counts[0];

        counts = next;
    }

    counts.iter().sum()
}

#[aoc(day6, part1)]
pub fn part1(input: &[usize]) -> usize {
    go(input, 80)
}

#[aoc(day6, part2)]
pub fn part2(input: &[usize]) -> usize {
    go(input, 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 5_934);
    }

    #[test]
    fn solution1() {
        assert_eq!(
            part1(&input_generator(include_str!("../input/2021/day6.txt"))),
            345_387,
        );
    }

    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(EXAMPLE)), 26_984_457_539);
    }

    #[test]
    fn solution2() {
        assert_eq!(
            part2(&input_generator(include_str!("../input/2021/day6.txt"))),
            1_574_445_493_136,
        );
    }
}
