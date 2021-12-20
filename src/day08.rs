use aoc_runner_derive::{aoc, aoc_generator};
use arrayvec::ArrayVec;
use itertools::Itertools;

type Digits = ArrayVec<char, 7>;

#[derive(Debug)]
pub struct Entry {
    sig_patterns: ArrayVec<Digits, 10>,
    output_digits: ArrayVec<Digits, 4>,
}

impl Entry {
    fn output(&self) -> u32 {
        todo!()
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Entry> {
    input
        .trim()
        .lines()
        .map(|s| {
            let (a, b) = s.split(" | ").collect_tuple().unwrap();
            Entry {
                sig_patterns: a.split(' ').map(|s| s.chars().sorted().collect()).collect(),
                output_digits: b.split(' ').map(|s| s.chars().sorted().collect()).collect(),
            }
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn part1(input: &[Entry]) -> usize {
    input
        .iter()
        .map(|entry| {
            entry
                .output_digits
                .iter()
                .filter(|output| matches!(output.len(), 2 | 3 | 4 | 7))
        })
        .flatten()
        .count()
}

#[aoc(day8, part2)]
pub fn part2(input: &[Entry]) -> u32 {
    input.iter().map(|entry| entry.output()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    const EXAMPLE_SHORT: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 26);
    }

    #[test]
    fn solution1() {
        assert_eq!(
            part1(&input_generator(include_str!("../input/2021/day8.txt"))),
            397,
        );
    }

    // #[test]
    // fn example2_short() {
    //     assert_eq!(part2(&input_generator(EXAMPLE_SHORT)), 5353);
    // }

    // #[test]
    // fn example2() {
    //     assert_eq!(part2(&input_generator(EXAMPLE)), 61229);
    // }

    // #[test]
    // fn solution2() {
    //     assert_eq!(
    //         part2(&input_generator(include_str!("../input/2021/day8.txt"))),
    //         100_727_924,
    //     );
    // }
}
