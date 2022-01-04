// Better solution from https://www.youtube.com/watch?v=uMWFaqQix6Y

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use ndarray::prelude::*;
use std::{ops::AddAssign, str::FromStr};

const N: usize = 26;

#[derive(Debug, Clone, Copy)]
pub struct Element(char);

impl Element {
    fn idx(self) -> usize {
        debug_assert!(self.0 >= 'A' && self.0 <= 'Z');
        self.0 as usize - 'A' as usize
    }
}

pub type Rule = (Element, Element, Element);
pub type Rules = Vec<Rule>;

#[derive(Debug, Clone)]
pub struct Template {
    singles: Array1<isize>,
    pairs: Array2<isize>,
}

impl Default for Template {
    fn default() -> Self {
        Self {
            singles: Array1::from_elem(N, 0),
            pairs: Array2::from_elem((N, N), 0),
        }
    }
}

impl FromStr for Template {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut singles = Array1::from_elem(N, 0);
        for c in s.chars().map(Element) {
            singles[c.idx()] += 1;
        }

        let mut pairs = Array2::from_elem((N, N), 0);
        for (a, b) in s.chars().map(Element).tuple_windows() {
            pairs[(a.idx(), b.idx())] += 1;
        }

        Ok(Self { singles, pairs })
    }
}

impl AddAssign<&Template> for Template {
    fn add_assign(&mut self, rhs: &Template) {
        self.singles += &rhs.singles;
        self.pairs += &rhs.pairs;
    }
}

impl Template {
    fn step(&self, rules: &[Rule]) -> Self {
        let mut next = Self::default();

        for (a, b, c) in rules {
            let count = self.pairs[(a.idx(), b.idx())];

            next.pairs[(a.idx(), b.idx())] -= count;
            next.pairs[(a.idx(), c.idx())] += count;
            next.pairs[(c.idx(), b.idx())] += count;
            next.singles[c.idx()] += count;
        }

        next += self;
        next
    }

    fn diff(&self) -> usize {
        let mut singles = self.singles.as_slice().unwrap().to_vec();
        let partition_idx = itertools::partition(&mut singles, |&c| c == 0);
        let (min, max) = singles[partition_idx..]
            .iter()
            .minmax()
            .into_option()
            .unwrap();
        (max - min) as usize
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> (Template, Rules) {
    let (template, rules) = input.trim().split("\n\n").collect_tuple().unwrap();

    let template = template.parse().unwrap();
    let rules = rules
        .lines()
        .map(|line| {
            let (pairs, elem) = line.split(" -> ").collect_tuple().unwrap();
            let (a, b) = pairs.chars().map(Element).collect_tuple().unwrap();

            (a, b, elem.chars().map(Element).next().unwrap())
        })
        .collect();

    (template, rules)
}

#[aoc(day14, part1)]
pub fn part1((template, rules): &(Template, Rules)) -> usize {
    let template = (0..10).fold(template.clone(), |template, _| template.step(rules));
    template.diff()
}

#[aoc(day14, part2)]
pub fn part2((template, rules): &(Template, Rules)) -> usize {
    let template = (0..40).fold(template.clone(), |template, _| template.step(rules));
    template.diff()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 1588);
    }

    #[test]
    fn solution1() {
        assert_eq!(
            part1(&input_generator(include_str!("../input/2021/day14.txt"))),
            3587,
        );
    }

    #[test]
    fn solution2() {
        assert_eq!(
            part2(&input_generator(include_str!("../input/2021/day14.txt"))),
            3906445077999,
        );
    }
}
