use std::collections::BTreeMap;

use pathfinding::prelude::dijkstra;
use rustc_hash::FxHashMap;
use utils::{FindSome, MatrixGet};

pub fn part1() -> usize {
    let map = include_str!("../inputs/2024/day20.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let start = map
        .iter()
        .enumerate()
        .find_some(|(y, r)| {
            r.iter()
                .enumerate()
                .find_some(|(x, c)| if *c == 'S' { Some((y, x)) } else { None })
        })
        .unwrap();
    let start = (start.0, start.1, 2);
    let end = map
        .iter()
        .enumerate()
        .find_some(|(y, r)| {
            r.iter()
                .enumerate()
                .find_some(|(x, c)| if *c == 'E' { Some((y, x)) } else { None })
        })
        .unwrap();
    macro_rules! d {
        ($map:expr) => {
            dijkstra(
                &start,
                |(y, x, d)| {
                    let mut res = Vec::new();
                    macro_rules! path {
                        ($ymod:expr, $xmod:expr) => {
                            if let Some(r) = $map.mget(*y, *x, $ymod, $xmod) {
                                let y = (*y as isize + $ymod) as usize;
                                let x = (*x as isize + $xmod) as usize;
                                if matches!(*r, '.' | 'E' | 'S') {
                                    res.push(((y, x, *d), 1));
                                }
                            }
                        };
                    }
                    path!(-1, 0);
                    path!(1, 0);
                    path!(0, -1);
                    path!(0, 1);
                    res
                },
                |(y, x, _)| (*y, *x) == end,
            )
            .unwrap()
        };
    }
    let mut ans: BTreeMap<usize, usize> = BTreeMap::default();
    let mut sum = 0;
    let (_, base) = d!(map);
    for y1 in 0..map.len() {
        for x1 in 0..map[y1].len() {
            let mut map = map.clone();
            map[y1][x1] = '.';
            let (_, len) = d!(map);
            let diff = base - len;
            *ans.entry(diff).or_default() += 1;
            if diff >= 100 {
                sum += 1;
            }
        }
    }
    println!("{ans:#?}");

    sum
}

const fn count_helper<const N: usize>(_: [(); N]) -> usize {
    N
}
macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

macro_rules! count_tts {
    ($($smth:tt)*) => {
        count_helper([$(replace_expr!($smth ())),*])
    }
}
// https://veykril.github.io/tlborm/decl-macros/building-blocks/counting.html
pub fn part2() -> i64 {
    let map = include_str!("../inputs/2024/day20.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let start = map
        .iter()
        .enumerate()
        .find_some(|(y, r)| {
            r.iter()
                .enumerate()
                .find_some(|(x, c)| if *c == 'S' { Some((y, x)) } else { None })
        })
        .unwrap();
    let start = (start.0, start.1, 2);
    let end = map
        .iter()
        .enumerate()
        .find_some(|(y, r)| {
            r.iter()
                .enumerate()
                .find_some(|(x, c)| if *c == 'E' { Some((y, x)) } else { None })
        })
        .unwrap();
    macro_rules! d {
        ($map:expr) => {
            dijkstra(
                &start,
                |(y, x, d)| {
                    let mut res = Vec::new();
                    macro_rules! path {
                        ($ymod:expr, $xmod:expr) => {
                            if let Some(r) = $map.mget(*y, *x, $ymod, $xmod) {
                                let y = (*y as isize + $ymod) as usize;
                                let x = (*x as isize + $xmod) as usize;
                                if matches!(*r, '.' | 'E' | 'S') {
                                    res.push(((y, x, *d), 1));
                                }
                            }
                        };
                    }
                    path!(-1, 0);
                    path!(1, 0);
                    path!(0, -1);
                    path!(0, 1);
                    res
                },
                |(y, x, _)| (*y, *x) == end,
            )
            .unwrap()
        };
    }
    let mut ans: BTreeMap<usize, usize> = BTreeMap::default();
    let mut sum = 0;
    let (_, base) = d!(map);
    macro_rules! rec {
        ($stmt:stmt) => {$stmt};
        ($count:expr, $($tail:tt)*) => {
            paste::item! {
                for [< y $count >] in 0..map.len() {
                    for [< x $count >] in 0..map[[< y $count >]].len() {
                        rec!($($tail)*);
                    }
                }
            }
        };
    }
    rec!(
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
        14,
        15,
        16,
        17,
        18,
        19,
        20,
        {}
    );
    for y1 in 0..map.len() {
        for x1 in 0..map[y1].len() {
            let mut map = map.clone();
            map[y1][x1] = '.';
            let (_, len) = d!(map);
            let diff = base - len;
            if diff >= 50 {
                *ans.entry(diff).or_default() += 1;
                sum += 1;
            }
        }
    }
    println!("{ans:#?}");

    sum
}
