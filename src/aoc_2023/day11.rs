use crate::utils::{load_string, NumTupleExt};
use std::collections::{BTreeSet, HashSet};

fn row_empty(r: usize, mat: &[Vec<char>]) -> bool {
    mat[r].iter().all(|c| *c == '.')
}
fn col_empty(c: usize, mat: &[Vec<char>]) -> bool {
    (0..mat.len()).all(|r| mat[r][c] == '.')
}

fn solver<const SIZE: usize>() -> usize {
    let input = load_string("inputs/2023/day11.input")
        .lines()
        .map(|r| r.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut y_mod = 0;
    let mut gs = Vec::new();
    for (y, r) in input.iter().enumerate() {
        if row_empty(y, &input) {
            y_mod += SIZE;
            continue;
        }
        let mut x_mod = 0;
        for (x, c) in r.iter().enumerate() {
            if *c == '#' {
                gs.push((y + y_mod, x + x_mod));
            } else if col_empty(x, &input) {
                x_mod += SIZE;
            }
        }
    }
    gs.iter()
        .map(|p| {
            gs.iter()
                .filter(|s| *p != **s)
                .map(|s| p.manhattan_distance(s))
                .sum::<usize>()
        })
        .sum::<usize>()
        / 2
}

pub fn part1() -> usize {
    solver::<{ 2 - 1 }>()
}
pub fn part2() -> usize {
    solver::<{ 1000000 - 1 }>()
}
