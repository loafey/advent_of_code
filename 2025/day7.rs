use std::{
    cell::{LazyCell, RefCell},
    collections::{HashMap, HashSet},
};

use utils::MatrixGet;

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
                    '.' => Map::Empty,
                    '^' => Map::Split,
                    'S' => {
                        start_pos = (y, x);
                        Map::Empty
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
    let mut beams = HashSet::from([start]);
    let mut splits = 0;
    'outer: loop {
        let mut new_beams = HashSet::new();
        for (y, x) in beams {
            match map.mget(y, x, 1, 0) {
                Some(m) => match m {
                    Map::Empty => drop(new_beams.insert((y + 1, x))),
                    Map::Split => {
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

fn count_paths((y, x): (usize, usize), map: &[Vec<Map>]) -> usize {
    #[thread_local]
    static CACHE: LazyCell<RefCell<HashMap<(usize, usize), usize>>> =
        LazyCell::new(Default::default);
    if let Some(val) = CACHE.borrow().get(&(y, x)) {
        return *val;
    }
    let val = match map.mget(y + 1, x, 1, 0) {
        Some(m) => match m {
            Map::Empty => count_paths((y + 1, x), map),
            Map::Split => count_paths((y + 1, x - 1), map) + count_paths((y + 1, x + 1), map),
        },
        None => 1,
    };
    CACHE.borrow_mut().insert((y, x), val);
    val
}

pub fn part2() -> usize {
    let (map, start) = input();
    count_paths(start, &map)
}
