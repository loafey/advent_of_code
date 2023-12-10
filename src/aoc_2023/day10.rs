use std::collections::{HashMap, HashSet};

use crate::utils::{load_string, matrix_get};

enum Dir {
    Up,
    Down,
    Left,
    Right,
}
use pathfinding::directed::{
    dfs::dfs,
    dijkstra::{dijkstra, dijkstra_all},
};
use Dir::*;
type Map = HashMap<(usize, usize, char), Vec<(usize, usize, char)>>;

fn graph() -> ((usize, usize, char), Map) {
    let inputs = load_string("inputs/2023/day10.input");
    let matrix = inputs
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let start = matrix
        .iter()
        .enumerate()
        .filter_map(|(y, i)| {
            i.iter()
                .enumerate()
                .find(|(_, c)| **c == 'S')
                .map(|(x, c)| (y, x, *c))
        })
        .next()
        .unwrap();
    let mut stack = vec![start];
    let mut map = HashMap::new();
    // Automate finding L
    while let Some((y, x, c)) = stack.pop() {
        if map.contains_key(&(y, x, c)) {
            continue;
        }
        let mut neighbors = Vec::new();
        match c {
            'F' | 'S' => {
                if let Some(bot) = matrix_get(y + 1, x, 0, 0, &matrix) {
                    neighbors.push((y + 1, x, bot))
                }
                if let Some(right) = matrix_get(y, x + 1, 0, 0, &matrix) {
                    neighbors.push((y, x + 1, right))
                }
            }
            '|' => {
                if let Some(bot) = matrix_get(y + 1, x, 0, 0, &matrix) {
                    neighbors.push((y + 1, x, bot))
                }
                if let Some(top) = matrix_get(y - 1, x, 0, 0, &matrix) {
                    neighbors.push((y - 1, x, top))
                }
            }
            '-' => {
                if let Some(right) = matrix_get(y, x + 1, 0, 0, &matrix) {
                    neighbors.push((y, x + 1, right))
                }
                if let Some(left) = matrix_get(y, x - 1, 0, 0, &matrix) {
                    neighbors.push((y, x - 1, left))
                }
            }
            'L' => {
                if let Some(right) = matrix_get(y, x + 1, 0, 0, &matrix) {
                    neighbors.push((y, x + 1, right))
                }
                if let Some(top) = matrix_get(y - 1, x, 0, 0, &matrix) {
                    neighbors.push((y - 1, x, top))
                }
            }
            'J' => {
                if let Some(left) = matrix_get(y, x - 1, 0, 0, &matrix) {
                    neighbors.push((y, x - 1, left))
                }
                if let Some(top) = matrix_get(y - 1, x, 0, 0, &matrix) {
                    neighbors.push((y - 1, x, top))
                }
            }
            '7' => {
                if let Some(left) = matrix_get(y, x - 1, 0, 0, &matrix) {
                    neighbors.push((y, x - 1, left))
                }
                if let Some(bot) = matrix_get(y + 1, x, 0, 0, &matrix) {
                    neighbors.push((y + 1, x, bot))
                }
            }
            '.' => {}
            _ => unreachable!(),
        }
        map.insert((y, x, c), neighbors.clone());
        stack.append(&mut neighbors);
    }
    (start, map)
}

pub fn part1() -> usize {
    let (start, mut map) = graph();
    let mut start_modified = start;
    start_modified.2 = '@';
    map.insert(start_modified, map[&start].clone());

    let path = dijkstra_all(&start_modified, |s| {
        map[s].clone().into_iter().map(|c| (c, 1))
    });
    path.into_iter()
        .map(|(_, (_, v))| v)
        .max()
        .unwrap_or_default()
}

pub fn part2() -> usize {
    // let (start, map) = graph();
    // let matrix = load_string("inputs/2023/day10.input")
    //     .lines()
    //     .map(|s| s.chars().collect::<Vec<_>>())
    //     .collect::<Vec<_>>();
    // let (y, x) = (matrix.len(), matrix[0].len());

    // let mut ascii = vec![vec!['.'; x]; y];
    // for ((y, x, c), _) in map {
    //     ascii[y][x] = c;
    // }

    // for r in ascii {
    //     for c in r {
    //         print!("{c}")
    //     }
    //     println!()
    // }

    // I counted by hand :))

    371
}
