use Map::*;
use std::collections::{BTreeSet, HashMap, HashSet};
use utils::{MatrixGet, Run};

enum Map {
    Empty,
    Split,
}

fn input() -> (Vec<Vec<Map>>, (usize, usize)) {
    let mut start_pos = (0, 0);
    let matrix = include_str!("../inputs/2025/day7.input")
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Empty,
                    '^' => Split,
                    'S' => {
                        start_pos = (y, x);
                        Empty
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    (matrix, start_pos)
}

pub fn part1() -> u64 {
    let (map, start) = input();
    let mut beams = BTreeSet::from([start]);
    let mut splits = 0;
    'outer: loop {
        let mut new_beams = BTreeSet::new();
        for (y, x) in beams {
            match map.mget(y, x, 1, 0) {
                Some(m) => match m {
                    Empty => drop(new_beams.insert((y + 1, x))),
                    Split => {
                        new_beams.insert((y + 1, x - 1));
                        new_beams.insert((y + 1, x + 1));
                        splits += 1;
                    }
                },
                None => break 'outer,
            }
        }
        beams = new_beams
    }
    splits
}

fn count_paths(
    (y, x): (usize, usize),
    map: &[Vec<Map>],
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(val) = cache.get(&(y, x)) {
        return *val;
    }
    let val = match map.mget(y + 1, x, 1, 0) {
        Some(m) => match m {
            Empty => count_paths((y + 1, x), map, cache),
            Split => {
                count_paths((y + 1, x - 1), map, cache) + count_paths((y + 1, x + 1), map, cache)
            }
        },
        None => 1,
    };
    cache.insert((y, x), val);
    val
}

pub fn part2() -> usize {
    input().run(|(m, s)| count_paths(s, &m, &mut HashMap::new()))
}
