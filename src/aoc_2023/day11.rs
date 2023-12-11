use std::collections::BTreeSet;

use crate::utils::{load_string, NumTupleExt};

fn solver(size: usize) -> usize {
    let mut input = load_string("inputs/2023/day11.input")
        .lines()
        .map(|r| r.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut y_gaps = BTreeSet::new();
    let mut r = 0;
    while r < input.len() {
        if input[r].iter().all(|c| *c == '.') {
            y_gaps.insert(r);
        }
        r += 1;
    }
    let mut x_gaps = BTreeSet::new();
    let mut c = 0;
    while c < input[0].len() {
        if (0..input.len()).all(|r| input[r][c] == '.') {
            x_gaps.insert(c);
        }
        c += 1;
    }

    let galaxies = input
        .into_iter()
        .enumerate()
        .flat_map(|(y, r)| {
            r.into_iter()
                .enumerate()
                .filter(|(x, c)| *c == '#')
                .map(move |(x, _)| (y, x))
        })
        .collect::<Vec<_>>();
    galaxies
        .iter()
        .map(|p| {
            galaxies
                .iter()
                .filter(|s| *p != **s)
                .map(|s| {
                    p.manhattan_distance(s)
                        + (((p.0.min(s.0))..(p.0.max(s.0)))
                            .filter_map(|y| y_gaps.get(&y))
                            .count()
                            * (size - 1))
                        + (((p.1.min(s.1))..(p.1.max(s.1)))
                            .filter_map(|x| x_gaps.get(&(x)))
                            .count()
                            * (size - 1))
                })
                .sum::<usize>()
        })
        .sum::<usize>()
        / 2
}

pub fn part1() -> usize {
    solver(2)
}
pub fn part2() -> usize {
    solver(1000000)
}
