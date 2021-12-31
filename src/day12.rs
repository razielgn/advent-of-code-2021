use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::{collections::BTreeMap, str::FromStr};

pub type Nodes = BTreeMap<Cave, Vec<Cave>>;
type Path<'a> = Vec<&'a Cave>;

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum Cave {
    Start,
    End,
    Small(String),
    Big(String),
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

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Nodes {
    input
        .trim()
        .lines()
        .map(|s| s.split('-').collect_tuple().unwrap())
        .fold(BTreeMap::new(), |mut nodes, (a, b)| {
            if b != "start" {
                nodes
                    .entry(a.parse().unwrap())
                    .or_default()
                    .push(b.parse().unwrap());
            }

            if a != "start" && b != "end" {
                nodes
                    .entry(b.parse().unwrap())
                    .or_default()
                    .push(a.parse().unwrap());
            }

            nodes
        })
}

fn can_visit<'a>(path: &[&'a Cave], cave: &'a Cave) -> bool {
    if let Cave::Small(_) = cave {
        !path.contains(&cave)
    } else {
        true
    }
}

fn all_paths_from<'a>(nodes: &'a Nodes, mut path: Path<'a>, current: &'a Cave) -> Vec<Path<'a>> {
    path.push(current);

    if let Cave::End = current {
        return vec![path];
    }

    nodes[current]
        .iter()
        .filter(|next_cave| can_visit(&path, next_cave))
        .map(|next_cave| all_paths_from(nodes, path.clone(), next_cave))
        .flatten()
        .collect_vec()
}

#[aoc(day12, part1)]
pub fn part1(nodes: &Nodes) -> usize {
    all_paths_from(nodes, vec![], &Cave::Start).len()
}

#[aoc(day12, part2)]
pub fn part2(_nodes: &Nodes) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            part1(&input_generator(
                "start-A
start-b
A-c
A-b
b-d
A-end
b-end"
            )),
            10
        );
    }

    #[test]
    fn example1_2() {
        assert_eq!(
            part1(&input_generator(
                "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"
            )),
            19
        );
    }

    #[test]
    fn example1_3() {
        assert_eq!(
            part1(&input_generator(
                "fs-end
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
start-RW"
            )),
            226
        );
    }

    #[test]
    fn solution1() {
        assert_eq!(
            part1(&input_generator(include_str!("../input/2021/day12.txt"))),
            5457,
        );
    }

    // #[test]
    // fn example2() {
    //     assert_eq!(part2(&input_generator(EXAMPLE)), 61229);
    // }

    // #[test]
    // fn solution2() {
    //     assert_eq!(
    //         part2(&input_generator(include_str!("../input/2021/day12.txt"))),
    //         1_027_422,
    //     );
    // }
}
