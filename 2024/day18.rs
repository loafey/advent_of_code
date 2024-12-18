use pathfinding::prelude::dijkstra;
use utils::{bi_functors::BiFunctorExtExt, MatrixGet};

pub fn part1() -> usize {
    let coords = include_str!("../inputs/2024/day18.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split_once(',')
                .unwrap()
                .splet(|s| s.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut grid = [['.'; 71]; 71];
    let start = (0, 0);
    let end = (70, 70);

    for (i, (x, y)) in coords.iter().cycle().enumerate() {
        if i >= 1024 {
            break;
        }
        grid[*y][*x] = '#';
    }

    let (path, len) = dijkstra(
        &start,
        |(y, x)| {
            let mut r = Vec::new();
            if let Some('.') = grid.mget(*y, *x, -1, 0) {
                r.push(((y - 1, *x), 1));
            }
            if let Some('.') = grid.mget(*y, *x, 1, 0) {
                r.push(((y + 1, *x), 1));
            }
            if let Some('.') = grid.mget(*y, *x, 0, -1) {
                r.push(((*y, x - 1), 1));
            }
            if let Some('.') = grid.mget(*y, *x, 0, 1) {
                r.push(((*y, x + 1), 1));
            }
            r
        },
        |c| *c == end,
    )
    .unwrap();

    for (y, x) in path {
        grid[y][x] = 'O'
    }
    for r in grid {
        for c in r {
            print!("{c}");
        }
        println!()
    }

    len
}
pub fn part2() -> String {
    let coords = include_str!("../inputs/2024/day18.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split_once(',')
                .unwrap()
                .splet(|s| s.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut grid = [['.'; 71]; 71];
    let start = (0, 0);
    let end = (70, 70);

    for (i, (x, y)) in coords.iter().cycle().enumerate() {
        grid[*y][*x] = '#';

        if dijkstra(
            &start,
            |(y, x)| {
                let mut r = Vec::new();
                if let Some('.') = grid.mget(*y, *x, -1, 0) {
                    r.push(((y - 1, *x), 1));
                }
                if let Some('.') = grid.mget(*y, *x, 1, 0) {
                    r.push(((y + 1, *x), 1));
                }
                if let Some('.') = grid.mget(*y, *x, 0, -1) {
                    r.push(((*y, x - 1), 1));
                }
                if let Some('.') = grid.mget(*y, *x, 0, 1) {
                    r.push(((*y, x + 1), 1));
                }
                r
            },
            |c| *c == end,
        )
        .is_none()
        {
            return format!("{x},{y}");
        }
    }
    "oops".to_string()
}
