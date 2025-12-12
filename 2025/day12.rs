use rayon::{iter, prelude::*};
use std::{collections::HashSet, io::Write, path::PathBuf};
use utils::{MatrixGet, MatrixTrans};

#[allow(clippy::type_complexity)]
fn input() -> (Vec<Vec<Vec<bool>>>, Vec<((usize, usize), Vec<usize>)>) {
    let mut shapes = Vec::new();
    let mut goals = Vec::new();
    for block in include_str!("../inputs/2025/day12.input").split("\n\n") {
        if block.contains('#') {
            shapes.push(
                block
                    .lines()
                    .skip(1)
                    .map(|l| l.chars().map(|c| c == '#').collect())
                    .collect(),
            )
        } else {
            for l in block.lines() {
                let (size, indexes) = l.split_once(':').unwrap();
                let indexes = indexes
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                let (x, y) = size.split_once('x').unwrap();
                let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
                goals.push(((x, y), indexes));
            }
        }
    }
    (shapes, goals)
}

fn flip_hor(mut shape: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    for l in shape.iter_mut() {
        l.reverse();
    }
    shape
}

fn rotate(shape: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    flip_hor(shape.transpose())
}

fn flip_vert(shape: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    flip_hor(rotate(rotate(shape)))
}

fn apply(
    xb: usize,
    yb: usize,
    shape: &[Vec<bool>],
    mut matrix: Vec<Vec<bool>>,
) -> Option<Vec<Vec<bool>>> {
    for (y, l) in shape.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if !*c {
                continue;
            }
            if let Some(false) = matrix.mget(yb, xb, y as isize, x as isize) {
                matrix[yb + y][xb + x] = *c;
            } else {
                return None;
            }
        }
    }
    Some(matrix)
}

fn solve(matrix: Vec<Vec<bool>>, shapes: &[Vec<Vec<bool>>], goals: &[(usize, usize)]) -> bool {
    let free = count_free(&matrix);
    if goals.iter().any(|(_, v)| free < *v) {
        return false;
    }
    if goals.is_empty() {
        return true;
    }
    let (index, goals) = (goals[0].0, &goals[1..]);

    let shape = shapes[index].clone();
    #[allow(clippy::type_complexity)]
    let opps: [fn(Vec<Vec<bool>>) -> Vec<Vec<bool>>; _] = [
        |s| s,
        |s| flip_hor(s),
        |s| flip_vert(s),
        |s| rotate(s),
        |s| flip_hor(rotate(s)),
        |s| flip_vert(rotate(s)),
        |s| rotate(rotate(s)),
        |s| flip_hor(rotate(rotate(s))),
        |s| flip_vert(rotate(rotate(s))),
        |s| rotate(rotate(rotate(s))),
        |s| flip_hor(rotate(rotate(rotate(s)))),
        |s| flip_vert(rotate(rotate(rotate(s)))),
    ];
    opps.into_iter()
        .any(|opp| with_shape(matrix.clone(), opp(shape.clone()), shapes, goals))
}

fn with_shape(
    matrix: Vec<Vec<bool>>,
    shape: Vec<Vec<bool>>,
    shapes: &[Vec<Vec<bool>>],
    goals: &[(usize, usize)],
) -> bool {
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if let Some(new) = apply(x, y, &shape, matrix.clone())
                && solve(new, shapes, goals)
            {
                return true;
            }
        }
    }
    false
}

fn count_free(shape: &[Vec<bool>]) -> usize {
    let mut o = 0;
    for l in shape {
        for c in l {
            if !*c {
                o += 1;
            }
        }
    }
    o
}
fn count_taken(shape: &[Vec<bool>]) -> usize {
    let mut o = 0;
    for l in shape {
        for c in l {
            if *c {
                o += 1;
            }
        }
    }
    o
}

pub fn part1() -> u64 {
    let (shapes, goals) = input();
    let goals: Vec<(_, Vec<usize>)> = goals
        .into_iter()
        .map(|(matrix, goals)| {
            (
                matrix,
                goals
                    .into_iter()
                    .enumerate()
                    .flat_map(|(i, a)| vec![i; a])
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    let goals = goals
        .into_iter()
        .map(|(coord, v)| {
            let mut total_size = 0;
            (
                coord,
                v.into_iter()
                    .map(|i| {
                        total_size += count_taken(&shapes[i]);
                        (i, total_size)
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    goals
        .into_iter()
        .enumerate()
        .map(|(i, ((x, y), indexes))| {
            let matrix = vec![vec![false; x]; y];
            let ans = solve(matrix, &shapes, &indexes) as u64;
            println!("{i}: {ans}");
            ans
        })
        .sum()
}

pub fn part2() -> u64 {
    unimplemented!()
}
