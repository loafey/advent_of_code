use std::collections::HashSet;

use crate::utils::{load_string, manhattan_distance};

pub fn part1() -> isize {
    let mut input = load_string("inputs/2023/day11.input")
        .lines()
        .map(|r| r.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut r = 0;
    while r < input.len() {
        if input[r].iter().all(|c| *c == '.') {
            input.insert(r, input[r].clone());
            r += 1;
        }
        r += 1;
    }
    let mut c = 0;
    while c < input[0].len() {
        if (0..(input.len())).all(|r| input[r][c] == '.') {
            input.iter_mut().for_each(|v| v.insert(c, '.'));
            c += 1;
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
                .map(move |(x, _)| (y as isize, x as isize))
        })
        .collect::<Vec<_>>();
    galaxies
        .iter()
        .map(|p| {
            galaxies
                .iter()
                .filter(|s| *p != **s)
                .map(|s| manhattan_distance(*p, *s))
                .sum::<isize>()
        })
        .sum::<isize>()
        / 2
}
pub fn part2() -> isize {
    let mut input = load_string("inputs/2023/day11.input")
        .lines()
        .map(|r| r.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut y_gaps = HashSet::new();
    let mut r = 0;
    while r < input.len() {
        if input[r].iter().all(|c| *c == '.') {
            y_gaps.insert(r);
        }
        r += 1;
    }
    let mut x_gaps = HashSet::new();
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
                .map(move |(x, _)| (y as isize, x as isize))
        })
        .collect::<Vec<_>>();
    galaxies
        .iter()
        .map(|p| {
            galaxies
                .iter()
                .filter(|s| *p != **s)
                .map(|s| {
                    manhattan_distance(*p, *s)
                        + (((p.0.min(s.0))..(p.0.max(s.0)))
                            .filter_map(|y| y_gaps.get(&(y as usize)))
                            .count() as isize
                            * (1000000 - 1))
                        + (((p.1.min(s.1))..(p.1.max(s.1)))
                            .filter_map(|x| x_gaps.get(&(x as usize)))
                            .count() as isize
                            * (1000000 - 1))
                })
                .sum::<isize>()
        })
        .sum::<isize>()
        / 2
}
