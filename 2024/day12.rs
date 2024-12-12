use rayon::prelude::*;
use rustc_hash::FxHashMap;
use std::sync::Arc;
use utils::MatrixGet;
matrixy::matrixy!("../inputs/2024/day12.input");

fn find_shapes(
    blowup: bool,
) -> impl ParallelIterator<Item = (FxHashMap<(usize, usize), i32>, usize)> {
    let map = Arc::new(if !blowup {
        include_str!("../inputs/2024/day12.input")
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().map(|c| c as u8).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    } else {
        let mut map = include_str!("../inputs/2024/day12.input")
            .lines()
            .filter(|l| !l.is_empty())
            .flat_map(|l| {
                let s = l
                    .chars()
                    .flat_map(|c| [c as u8, c as u8, c as u8])
                    .collect::<Vec<_>>();
                [s.clone(), s.clone(), s]
            })
            .collect::<Vec<_>>();
        map.iter_mut().for_each(|r| {
            r.insert(0, 0);
            r.push(0);
        });
        map.insert(0, vec![0; map[0].len()]);
        map.push(vec![0; map[0].len()]);
        map
    });

    (b'A'..=b'Z')
        .into_par_iter()
        .filter_map(move |c| {
            let mut new_map = vec![vec![0; map[0].len()]; map.len()];
            let mut found = false;
            for y in 0..map.len() {
                for x in 0..map[0].len() {
                    if map[y][x] == c {
                        new_map[y][x] = c;
                        found = true;
                    }
                }
            }
            if found {
                Some(new_map)
            } else {
                None
            }
        })
        .flat_map(move |mut map| {
            let mut res = Vec::new();
            for y in 0..map.len() {
                for x in 0..map[y].len() {
                    let c = map[y][x];
                    if c == 0 {
                        continue;
                    }
                    let mut stack = vec![(y, x)];
                    let mut to_remove = Vec::new();
                    while let Some((y, x)) = stack.pop() {
                        to_remove.push((y, x));
                        if let Some(v) = map.mget(y, x, -1, 0) {
                            if *v == c && !to_remove.contains(&(y - 1, x)) {
                                stack.push((y - 1, x))
                            }
                        };
                        if let Some(v) = map.mget(y, x, 1, 0) {
                            if *v == c && !to_remove.contains(&(y + 1, x)) {
                                stack.push((y + 1, x))
                            }
                        };
                        if let Some(v) = map.mget(y, x, 0, -1) {
                            if *v == c && !to_remove.contains(&(y, x - 1)) {
                                stack.push((y, x - 1))
                            }
                        };
                        if let Some(v) = map.mget(y, x, 0, 1) {
                            if *v == c && !to_remove.contains(&(y, x + 1)) {
                                stack.push((y, x + 1))
                            }
                        };
                    }

                    to_remove.sort();
                    to_remove.dedup();
                    let area = to_remove.len();
                    let mut perimiter = FxHashMap::default();
                    let mut not_interesting = Vec::new();
                    for (y, x) in to_remove {
                        map[y][x] = 0;
                        perimiter.insert((y, x), 0);
                        not_interesting.push((y, x));
                        if blowup {
                            perimiter.entry((y - 1, x)).or_insert(1);
                            perimiter.entry((y + 1, x)).or_insert(1);
                            perimiter.entry((y, x - 1)).or_insert(1);
                            perimiter.entry((y, x + 1)).or_insert(1);
                            perimiter.entry((y - 1, x - 1)).or_insert(1);
                            perimiter.entry((y + 1, x - 1)).or_insert(1);
                            perimiter.entry((y + 1, x + 1)).or_insert(1);
                            perimiter.entry((y - 1, x + 1)).or_insert(1);
                        } else {
                            *perimiter.entry((y - 1, x)).or_insert(0) += 1;
                            *perimiter.entry((y + 1, x)).or_insert(0) += 1;
                            *perimiter.entry((y, x - 1)).or_insert(0) += 1;
                            *perimiter.entry((y, x + 1)).or_insert(0) += 1;
                        }
                    }
                    for (y, x) in not_interesting {
                        perimiter.remove(&(y, x));
                    }
                    res.push((perimiter, area));
                }
            }
            res
        })
}

pub fn part1() -> usize {
    find_shapes(false)
        .map(|(perimiter, area)| {
            area * perimiter
                .into_values()
                .filter(|c| *c <= 4)
                .map(|s| s as usize)
                .sum::<usize>()
        })
        .sum()
}
pub fn part2() -> usize {
    let perimeters = find_shapes(true);

    perimeters
        .into_par_iter()
        .map(|(mut perimiter, area)| {
            let mut sides = 0;

            while !perimiter.is_empty() {
                let mut keys = perimiter.keys().collect::<Vec<_>>();
                keys.sort();
                let mut stack = vec![*keys[0]];
                let (mut dy, mut dx) = (0i32, 1i32);

                while let Some((y, x)) = stack.pop() {
                    perimiter.remove(&(y, x));
                    let m = (y + dy as usize, x + dx as usize);
                    if perimiter.contains_key(&m) {
                        stack.push(m);
                    } else {
                        if dx != 0 {
                            if perimiter.contains_key(&(y - 1, x)) {
                                stack.push((y - 1, x));
                                dx = 0;
                                dy = -1;
                            } else if perimiter.contains_key(&(y + 1, x)) {
                                stack.push((y + 1, x));
                                dx = 0;
                                dy = 1;
                            }
                        } else if perimiter.contains_key(&(y, x - 1)) {
                            stack.push((y, x - 1));
                            dx = -1;
                            dy = 0;
                        } else if perimiter.contains_key(&(y, x + 1)) {
                            stack.push((y, x + 1));
                            dx = 1;
                            dy = 0;
                        }
                        sides += 1;
                    }
                }
            }

            let area = area / 9;
            area * sides
        })
        .sum()
}
