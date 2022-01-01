use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::{collections::BTreeMap, str::FromStr};

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum Cave {
    Start,
    End,
    Small(String),
    Big(String),
}

impl Cave {
    fn is_small(&self) -> bool {
        matches!(self, Cave::Small(_))
    }
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "start" => Cave::Start,
            "end" => Cave::End,
            _ => {
                if s.chars().all(|c| c.is_ascii_lowercase()) {
                    Cave::Small(s.into())
                } else {
                    Cave::Big(s.into())
                }
            }
        })
    }
}

pub type Nodes = BTreeMap<Cave, Vec<Cave>>;

trait Path<'a>: Clone {
    fn visit(&mut self, cave: &'a Cave);
    fn can_visit(&self, cave: &'a Cave) -> bool;
}

#[derive(Clone, Default)]
struct Part1Path<'a>(Vec<&'a Cave>);

impl<'a> Path<'a> for Part1Path<'a> {
    fn visit(&mut self, cave: &'a Cave) {
        self.0.push(cave);
    }

    fn can_visit(&self, cave: &'a Cave) -> bool {
        if !cave.is_small() {
            return true;
        }

        !self.0.contains(&cave)
    }
}

#[derive(Clone, Default)]
struct Part2Path<'a> {
    any_small_cave_visited_twice: bool,
    path: Vec<&'a Cave>,
}

impl Part2Path<'_> {
    fn times_visited(&self, cave: &Cave) -> usize {
        self.path.iter().filter(|&&visited| visited == cave).count()
    }
}

impl<'a> Path<'a> for Part2Path<'a> {
    fn visit(&mut self, cave: &'a Cave) {
        self.path.push(cave);

        if cave.is_small() && self.times_visited(cave) == 2 {
            self.any_small_cave_visited_twice = true;
        }
    }

    fn can_visit(&self, cave: &'a Cave) -> bool {
        if !cave.is_small() || !self.path.contains(&cave) {
            return true;
        }

        !self.any_small_cave_visited_twice
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Nodes {
    input
        .trim()
        .lines()
        .map(|s| s.split('-').collect_tuple().unwrap())
        .fold(BTreeMap::new(), |mut nodes, (a, b)| {
            let a: Cave = a.parse().unwrap();
            let b: Cave = b.parse().unwrap();

            if b != Cave::Start {
                nodes.entry(a.clone()).or_default().push(b.clone());
            }

            if a != Cave::Start && b != Cave::End {
                nodes.entry(b).or_default().push(a);
            }

            nodes
        })
}

fn count_all_paths_from<'a, P>(nodes: &'a Nodes, mut path: P, current: &'a Cave) -> usize
where
    P: Path<'a>,
{
    path.visit(current);

    if let Cave::End = current {
        return 1;
    }

    nodes[current]
        .iter()
        .filter(|next_cave| path.can_visit(next_cave))
        .map(|next_cave| count_all_paths_from(nodes, path.clone(), next_cave))
        .sum()
}

#[aoc(day12, part1)]
pub fn part1(nodes: &Nodes) -> usize {
    count_all_paths_from(nodes, Part1Path::default(), &Cave::Start)
}

#[aoc(day12, part2)]
pub fn part2(nodes: &Nodes) -> usize {
    count_all_paths_from(nodes, Part2Path::default(), &Cave::Start)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_1: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    static EXAMPLE_2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    static EXAMPLE_3: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn example1_1() {
        assert_eq!(part1(&input_generator(EXAMPLE_1)), 10);
    }

    #[test]
    fn example1_2() {
        assert_eq!(part1(&input_generator(EXAMPLE_2)), 19);
    }

    #[test]
    fn example1_3() {
        assert_eq!(part1(&input_generator(EXAMPLE_3)), 226);
    }

    #[test]
    fn solution1() {
        assert_eq!(
            part1(&input_generator(include_str!("../input/2021/day12.txt"))),
            5_457,
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(part2(&input_generator(EXAMPLE_1)), 36);
    }

    #[test]
    fn example2_2() {
        assert_eq!(part2(&input_generator(EXAMPLE_2)), 103);
    }

    #[test]
    fn example2_3() {
        assert_eq!(part2(&input_generator(EXAMPLE_3)), 3509);
    }

    #[test]
    fn solution2() {
        assert_eq!(
            part2(&input_generator(include_str!("../input/2021/day12.txt"))),
            128_506,
        );
    }
}
