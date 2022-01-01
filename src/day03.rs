use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_owned()).collect()
}

fn bin_iter_to_decimal(it: impl DoubleEndedIterator<Item = u32>) -> u32 {
    it.rev().zip(0..).map(|(n, pow)| n * 2_u32.pow(pow)).sum()
}

#[aoc(day3, part1)]
pub fn part1(input: &[String]) -> u32 {
    let width = input[0].len();
    let mut occurrences = vec![(0, 0); width];

    for n in input {
        for (c, mut occ) in n.chars().zip(occurrences.iter_mut()) {
            if let '0' = c {
                occ.0 += 1;
            } else {
                occ.1 += 1;
            }
        }
    }

    let gamma_rate = bin_iter_to_decimal(
        occurrences
            .iter()
            .map(|(zeroes, ones)| if zeroes > ones { 0 } else { 1 }),
    );

    let epsilon_rate = gamma_rate ^ ((1 << width) - 1);

    gamma_rate * epsilon_rate
}

#[aoc(day3, part2)]
pub fn part2(input: &[String]) -> u32 {
    fn find_oxygen_generator_rating(mut input: Vec<String>) -> u32 {
        for bit in 0.. {
            if input.len() == 1 {
                return u32::from_str_radix(&input[0], 2).unwrap();
            }

            let ones = itertools::partition(&mut input, |s| s.chars().nth(bit).unwrap() == '1');
            let zeroes = input.len() - ones;
            let split_index = ones;

            if ones >= zeroes {
                let _ = input.split_off(split_index);
            } else {
                input = input.split_off(split_index);
            }
        }

        unreachable!()
    }

    fn find_co2_scrubber_rating(mut input: Vec<String>) -> u32 {
        for bit in 0.. {
            if input.len() == 1 {
                return u32::from_str_radix(&input[0], 2).unwrap();
            }

            let zeroes = itertools::partition(&mut input, |s| s.chars().nth(bit).unwrap() == '0');
            let ones = input.len() - zeroes;
            let split_index = zeroes;

            if ones >= zeroes {
                let _ = input.split_off(split_index);
            } else {
                input = input.split_off(split_index);
            }
        }

        unreachable!()
    }

    let oxygen_generator_rating = find_oxygen_generator_rating(input.to_vec());
    let co2_scrubber_rating = find_co2_scrubber_rating(input.to_vec());

    oxygen_generator_rating * co2_scrubber_rating
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

    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(EXAMPLE)), 230);
    }

    #[test]
    fn solution2() {
        assert_eq!(
            part2(&input_generator(include_str!("../input/2021/day3.txt"))),
            793_873,
        );
    }
}
