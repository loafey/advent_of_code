use std::collections::BTreeMap;

use pathfinding::prelude::bfs;
use rustc_hash::{FxHashMap, FxHashSet};
use utils::{FindSome, MatrixGet};

fn find(map: &[Vec<char>], g: char) -> (usize, usize) {
    map.iter()
        .enumerate()
        .find_some(|(y, r)| {
            r.iter()
                .enumerate()
                .find_some(|(x, c)| if *c == g { Some((y, x)) } else { None })
        })
        .unwrap()
}

fn solve(r: usize) -> usize {
    let map = include_str!("../inputs/2024/day20.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let start = find(&map, 'S');
    let end = find(&map, 'E');

    let path = bfs(
        &start,
        |(y, x)| {
            let mut res = Vec::new();
            macro_rules! path {
                ($ymod:expr, $xmod:expr) => {
                    if let Some(r) = map.mget(*y, *x, $ymod, $xmod) {
                        let y = (*y as isize + $ymod) as usize;
                        let x = (*x as isize + $xmod) as usize;
                        if matches!(*r, '.' | 'E' | 'S') {
                            res.push((y, x));
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
        |(y, x)| (*y, *x) == end,
    )
    .unwrap();
    let pos = path
        .iter()
        .enumerate()
        .map(|(i, c)| (*c, i))
        .collect::<FxHashMap<_, _>>();
    let mut sum = 0;
    for (j, (y, x)) in path.iter().enumerate() {
        let mut check = FxHashSet::default();
        for r in 0..=r {
            for offset in 0..=r {
                let inv_offset = r - offset;
                check.insert(((y + offset, x + inv_offset), r));
                check.insert(((y + inv_offset, x - offset), r));
                check.insert(((y - offset, x - inv_offset), r));
                check.insert(((y - inv_offset, x + offset), r));
            }
        }
        for (c, r) in check {
            if let Some(i) = pos.get(&c) {
                if j >= *i {
                    continue;
                }
                let Some(diff) = (i - j).checked_sub(r) else {
                    continue;
                };
                if diff >= 100 {
                    sum += 1;
                }
            }
        }
    }

    sum
}

pub fn part1() -> usize {
    solve(2)
}

pub fn part2() -> usize {
    solve(20)
}
