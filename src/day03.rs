use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_owned()).collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[String]) -> u32 {
    let mut occurrences = vec![(0, 0); input[0].len()];

    for n in input {
        for (c, mut occ) in n.chars().zip(occurrences.iter_mut()) {
            if let '0' = c {
                occ.0 += 1;
            } else {
                occ.1 += 1;
            }
        }
    }

    let gamma_rate = u32::from_str_radix(
        &occurrences
            .iter()
            .map(|(zeroes, ones)| if zeroes > ones { '0' } else { '1' })
            .collect::<String>(),
        2,
    )
    .unwrap();

    let epsilon_rate = u32::from_str_radix(
        &occurrences
            .iter()
            .map(|(zeroes, ones)| if zeroes > ones { '1' } else { '0' })
            .collect::<String>(),
        2,
    )
    .unwrap();

    gamma_rate * epsilon_rate
}

#[aoc(day3, part2)]
pub fn part2(input: &[String]) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 198);
    }

    #[test]
    fn solution1() {
        assert_eq!(
            part1(&input_generator(include_str!("../input/2021/day3.txt"))),
            1_025_636
        );
    }

    // #[test]
    // fn example2() {
    //     assert_eq!(part2(&input_generator(EXAMPLE)), 900);
    // }

    // #[test]
    // fn solution2() {
    //     assert_eq!(
    //         part2(&input_generator(include_str!("../input/2021/day3.txt"))),
    //         1_942_068_080,
    //     );
    // }
}
