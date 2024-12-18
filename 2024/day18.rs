use pathfinding::prelude::dijkstra;
use std::collections::BTreeSet;
use utils::{bi_functors::BiFunctorExtExt, MatrixGet};

fn successors(grid: &[[char; 71]; 71], y: usize, x: usize) -> Vec<((usize, usize), usize)> {
    let mut r = Vec::new();
    if let Some('.') = grid.mget(y, x, -1, 0) {
        r.push(((y - 1, x), 1));
    }
    if let Some('.') = grid.mget(y, x, 1, 0) {
        r.push(((y + 1, x), 1));
    }
    if let Some('.') = grid.mget(y, x, 0, -1) {
        r.push(((y, x - 1), 1));
    }
    if let Some('.') = grid.mget(y, x, 0, 1) {
        r.push(((y, x + 1), 1));
    }
    r
}

const END: (usize, usize) = (70, 70);
pub fn part1() -> usize {
    let coords = include_str!("../inputs/2024/day18.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split_once(',')
                .unwrap()
                .splet(|s| s.parse::<usize>().unwrap())
        });

    let mut grid = [['.'; 71]; 71];

    for (i, (x, y)) in coords.enumerate() {
        if i >= 1024 {
            break;
        }
        grid[y][x] = '#';
    }

    dijkstra(&(0, 0), |(y, x)| successors(&grid, *y, *x), |c| *c == END)
        .unwrap()
        .1
}
pub fn part2() -> String {
    let coords = include_str!("../inputs/2024/day18.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split_once(',')
                .unwrap()
                .splet(|s| s.parse::<usize>().unwrap())
        });

    let mut grid = [['.'; 71]; 71];
    let mut path = BTreeSet::default();

    for (i, (x, y)) in coords.enumerate() {
        grid[y][x] = '#';
        if i < 2000 || (!path.is_empty() && !path.contains(&(y, x))) {
            continue;
        }

        if let Some(sp) = dijkstra(&(0, 0), |(y, x)| successors(&grid, *y, *x), |c| *c == END)
            .map(|(s, _)| BTreeSet::from_iter(s.into_iter()))
        {
            path = sp;
        } else {
            return format!("{x},{y}");
        }
    }
    String::new()
}
