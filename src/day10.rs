use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug)]
enum LineError {
    Incomplete(Vec<char>),
    Corrupted(char),
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.trim().lines().map(|s| s.to_owned()).collect()
}

fn open(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => unreachable!(),
    }
}

fn eval_line(s: &str) -> Result<(), LineError> {
    let mut stack = Vec::new();

    for c in s.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' if stack.last() == Some(&open(c)) => {
                let _ = stack.pop();
            }
            _ => {
                return Err(LineError::Corrupted(c));
            }
        }
    }

    if !stack.is_empty() {
        return Err(LineError::Incomplete(stack));
    }

    Ok(())
}

#[aoc(day10, part1)]
pub fn part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|line| eval_line(line))
        .filter_map(|res| match res {
            Err(LineError::Corrupted(')')) => Some(3),
            Err(LineError::Corrupted(']')) => Some(57),
            Err(LineError::Corrupted('}')) => Some(1_197),
            Err(LineError::Corrupted('>')) => Some(25_137),
            _ => None,
        })
        .sum()
}

fn score(v: &[char]) -> usize {
    v.iter()
        .rev()
        .map(|c| match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => unreachable!(),
        })
        .fold(0, |acc, points| acc * 5 + points)
}

#[aoc(day10, part2)]
pub fn part2(input: &[String]) -> usize {
    let scores = input
        .iter()
        .map(|line| eval_line(line))
        .filter_map(|res| {
            if let Err(LineError::Incomplete(stack)) = res {
                Some(score(&stack))
            } else {
                None
            }
        })
        .sorted()
        .collect_vec();

    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 26_397);
    }

    #[test]
    fn solution1() {
        assert_eq!(
            part1(&input_generator(include_str!("../input/2021/day10.txt"))),
            436_497,
        );
    }

    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(EXAMPLE)), 288_957);
    }

    #[test]
    fn solution2() {
        assert_eq!(
            part2(&input_generator(include_str!("../input/2021/day10.txt"))),
            2_377_613_374,
        );
    }
}
