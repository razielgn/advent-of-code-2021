use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn basins(grid: &[Vec<u8>]) -> impl Iterator<Item = (usize, usize)> + '_ {
    iproduct!(0..grid.len(), 0..grid[0].len()).filter(|&(h, w)| {
        let n = grid[h][w];

        [
            grid.get(h.wrapping_sub(1)).and_then(|row| row.get(w)),
            grid.get(h + 1).and_then(|row| row.get(w)),
            grid.get(h).and_then(|row| row.get(w.wrapping_sub(1))),
            grid.get(h).and_then(|row| row.get(w + 1)),
        ]
        .into_iter()
        .flatten()
        .all(|neighbour| neighbour > &n)
    })
}

#[aoc(day9, part1)]
pub fn part1(input: &[Vec<u8>]) -> u16 {
    basins(input).map(|(h, w)| (input[h][w] as u16) + 1).sum()
}

fn basin_size(grid: &[Vec<u8>], h: usize, w: usize) -> usize {
    fn go(g: &mut Vec<Vec<u8>>, h: usize, w: usize) -> usize {
        let n = g.get(h).and_then(|row| row.get(w));

        if let None | Some(9) | Some(255) = n {
            return 0;
        }

        g[h][w] = 255; // Mark as visited.

        1 + go(g, h.wrapping_sub(1), w)
            + go(g, h + 1, w)
            + go(g, h, w.wrapping_sub(1))
            + go(g, h, w + 1)
    }

    go(&mut grid.to_vec(), h, w)
}

#[aoc(day9, part2)]
pub fn part2(input: &[Vec<u8>]) -> usize {
    basins(input)
        .map(|(h, w)| basin_size(input, h, w))
        .sorted()
        .rev()
        .take(3)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 15);
    }

    #[test]
    fn solution1() {
        assert_eq!(
            part1(&input_generator(include_str!("../input/2021/day9.txt"))),
            560,
        );
    }

    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(EXAMPLE)), 1134);
    }

    #[test]
    fn solution2() {
        assert_eq!(
            part2(&input_generator(include_str!("../input/2021/day9.txt"))),
            959_136,
        );
    }
}
