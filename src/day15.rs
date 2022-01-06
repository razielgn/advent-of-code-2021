use aoc_runner_derive::{aoc, aoc_generator};
use ndarray::prelude::*;
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, collections::BTreeSet};

type Grid = Array2<u16>;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Grid {
    let rows = input.trim().lines().count();
    let cols = input.trim().lines().next().unwrap().len();

    Array::from_iter(
        input
            .trim()
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u16)),
    )
    .into_shape((rows, cols))
    .unwrap()
}

type Pos = (usize, usize);

fn neighbours<'a>(
    pos: &Pos,
    visited: &'a BTreeSet<Pos>,
    upper_bound: &'a Pos,
) -> impl Iterator<Item = Pos> + 'a {
    [
        (pos.0.wrapping_sub(1), pos.1),
        (pos.0, pos.1.wrapping_sub(1)),
        (pos.0 + 1, pos.1),
        (pos.0, pos.1 + 1),
    ]
    .into_iter()
    .filter(|&pos| pos.0 <= upper_bound.0 && pos.1 <= upper_bound.1)
    .filter(|pos| !visited.contains(pos))
}

#[aoc(day15, part1)]
pub fn part1(grid: &Grid) -> u16 {
    let dest = (grid.nrows() - 1, grid.ncols() - 1);
    let start = (0, 0);
    let mut visited = BTreeSet::new();
    let mut border = PriorityQueue::new();
    border.push(start, Reverse(0));

    while let Some((pos, risk)) = border.pop() {
        if pos == dest {
            return risk.0;
        }

        visited.insert(pos);

        for next_pos in neighbours(&pos, &visited, &dest) {
            let new_risk = Reverse(risk.0 + grid[next_pos]);

            match border.get_priority(&next_pos) {
                Some(&existing_risk) if existing_risk == Reverse(0) || existing_risk < new_risk => {
                    border.change_priority(&next_pos, new_risk);
                }
                Some(_) => {}
                None => {
                    border.push(next_pos, new_risk);
                }
            }
        }
    }

    unreachable!()
}

fn enlarge_grid(small_grid: &Grid) -> Grid {
    let tile_size = (small_grid.nrows(), small_grid.ncols());
    let scale = 5;
    let mut grid = Array2::from_elem((tile_size.0 * scale, tile_size.1 * scale), 0u16);

    let increase_risk = |risk, n| {
        let nrisk = risk + n;

        if nrisk > 9 {
            nrisk - 9
        } else {
            nrisk
        }
    };

    for i in 0..=(scale - 1) {
        let tile = small_grid.mapv(|risk| increase_risk(risk, i as u16));

        grid.slice_mut(s![
            i * tile.ncols()..(i + 1) * tile.ncols(),
            0..tile.nrows()
        ])
        .assign(&tile);
    }

    for i in 0..=(scale - 1) {
        for j in 1..=(scale - 1) {
            let tile = grid
                .slice(s![i * tile_size.0..(i + 1) * tile_size.0, 0..tile_size.1,])
                .mapv(|risk| increase_risk(risk, j as u16));

            grid.slice_mut(s![
                i * tile.ncols()..(i + 1) * tile.ncols(),
                j * tile.nrows()..(j + 1) * tile.nrows(),
            ])
            .assign(&tile);
        }
    }

    grid
}

#[aoc(day15, part2)]
pub fn part2(grid: &Grid) -> u16 {
    part1(&enlarge_grid(grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 40);
    }

    #[test]
    fn solution1() {
        assert_eq!(
            part1(&input_generator(include_str!("../input/2021/day15.txt"))),
            435,
        );
    }

    static EXAMPLE2: &str = "11637517422274862853338597396444961841755517295286
13813736722492484783351359589446246169155735727126
21365113283247622439435873354154698446526571955763
36949315694715142671582625378269373648937148475914
74634171118574528222968563933317967414442817852555
13191281372421239248353234135946434524615754563572
13599124212461123532357223464346833457545794456865
31254216394236532741534764385264587549637569865174
12931385212314249632342535174345364628545647573965
23119445813422155692453326671356443778246755488935
22748628533385973964449618417555172952866628316397
24924847833513595894462461691557357271266846838237
32476224394358733541546984465265719557637682166874
47151426715826253782693736489371484759148259586125
85745282229685639333179674144428178525553928963666
24212392483532341359464345246157545635726865674683
24611235323572234643468334575457944568656815567976
42365327415347643852645875496375698651748671976285
23142496323425351743453646285456475739656758684176
34221556924533266713564437782467554889357866599146
33859739644496184175551729528666283163977739427418
35135958944624616915573572712668468382377957949348
43587335415469844652657195576376821668748793277985
58262537826937364893714847591482595861259361697236
96856393331796741444281785255539289636664139174777
35323413594643452461575456357268656746837976785794
35722346434683345754579445686568155679767926678187
53476438526458754963756986517486719762859782187396
34253517434536462854564757396567586841767869795287
45332667135644377824675548893578665991468977611257
44961841755517295286662831639777394274188841538529
46246169155735727126684683823779579493488168151459
54698446526571955763768216687487932779859814388196
69373648937148475914825958612593616972361472718347
17967414442817852555392896366641391747775241285888
46434524615754563572686567468379767857948187896815
46833457545794456865681556797679266781878137789298
64587549637569865174867197628597821873961893298417
45364628545647573965675868417678697952878971816398
56443778246755488935786659914689776112579188722368
55172952866628316397773942741888415385299952649631
57357271266846838237795794934881681514599279262561
65719557637682166874879327798598143881961925499217
71484759148259586125936169723614727183472583829458
28178525553928963666413917477752412858886352396999
57545635726865674683797678579481878968159298917926
57944568656815567976792667818781377892989248891319
75698651748671976285978218739618932984172914319528
56475739656758684176786979528789718163989182927419
67554889357866599146897761125791887223681299833479";

    #[test]
    fn enlarge_grid_() {
        assert_eq!(
            enlarge_grid(&input_generator(EXAMPLE)),
            input_generator(EXAMPLE2),
        );
    }

    #[test]
    fn example2() {
        assert_eq!(part1(&input_generator(EXAMPLE2)), 315);
    }

    #[test]
    fn solution2() {
        assert_eq!(
            part2(&input_generator(include_str!("../input/2021/day15.txt"))),
            2_842,
        );
    }
}
