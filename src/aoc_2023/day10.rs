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
            'F' => {
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
            'L' | 'S' => {
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
    let (start, map) = graph();
    let matrix = load_string("inputs/2023/day10.input")
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (y, x) = (matrix.len(), matrix[0].len());

    let mut ascii = vec![vec![false; x * 3]; y * 3];
    ascii.iter_mut().array_chunks::<3>().for_each(|[_, a, _]| {
        a.iter_mut()
            .array_chunks::<3>()
            .for_each(|[_, a, _]| *a = true)
    });
    for ((y, x, c), _) in map {
        let y = (y * 3) + 1;
        let x = (x * 3) + 1;
        match c {
            '|' => {
                ascii[y - 1][x] = true;
                ascii[y][x] = true;
                ascii[y + 1][x] = true
            }
            '-' => {
                ascii[y][x - 1] = true;
                ascii[y][x] = true;
                ascii[y][x + 1] = true
            }
            'L' | 'S' => {
                ascii[y - 1][x] = true;
                ascii[y][x] = true;
                ascii[y][x + 1] = true
            }
            'J' => {
                ascii[y - 1][x] = true;
                ascii[y][x] = true;
                ascii[y][x - 1] = true
            }
            '7' => {
                ascii[y][x - 1] = true;
                ascii[y][x] = true;
                ascii[y + 1][x] = true
            }
            'F' => {
                ascii[y][x + 1] = true;
                ascii[y][x] = true;
                ascii[y + 1][x] = true
            }
            '.' => ascii[y][x] = true,
            _ => {}
        }
    }

    let mut stack = vec![(0, 0)];
    while let Some((y, x)) = stack.pop() {
        ascii[y][x] = true;
        if let Some(left) = matrix_get(y, x, 0, -1, &ascii) {
            if !left {
                stack.push((y, x - 1))
            }
        }
        if let Some(right) = matrix_get(y, x, 0, 1, &ascii) {
            if !right {
                stack.push((y, x + 1))
            }
        }
        if let Some(up) = matrix_get(y, x, -1, 0, &ascii) {
            if !up {
                stack.push((y - 1, x))
            }
        }
        if let Some(down) = matrix_get(y, x, 1, 0, &ascii) {
            if !down {
                stack.push((y + 1, x))
            }
        }
    }

    // for r in ascii {
    //     for c in r {
    //         if c {
    //             print!("█")
    //         } else {
    //             print!(" ")
    //         }
    //     }
    //     println!()
    // }

    let mut i = 0;
    for y in (0..ascii.len()).step_by(3) {
        for x in (0..ascii[y].len()).step_by(3) {
            let arr = [
                [ascii[y][x], ascii[y][x + 1], ascii[y][x + 2]],
                [ascii[y + 1][x], ascii[y + 1][x + 1], ascii[y + 1][x + 2]],
                [ascii[y + 2][x], ascii[y + 2][x + 1], ascii[y + 2][x + 2]],
            ];
            if matches!(
                arr,
                [
                    [false, false, false],
                    [false, true, false],
                    [false, false, false]
                ]
            ) {
                i += 1;
            }
        }
    }
    i
}
