use std::collections::{BTreeMap, HashSet};
use utils::load_string;

fn char_to_height(c: char) -> isize {
    (c as u8 - 97) as isize
}

fn get_neighbors(coord: (usize, usize), grid: &[Vec<isize>]) -> Vec<(usize, usize)> {
    // HORRID
    [-1, 0, 1]
        .into_iter()
        .flat_map(|x| {
            [-1, 0, 1].into_iter().filter_map(move |y| {
                if x == 0 && y == 0 {
                    None
                } else if x == 0 || y == 0 {
                    let new_y = (coord.1 as isize + y) as usize;
                    let new_x = (coord.0 as isize + x) as usize;
                    if new_y < grid.len() && new_x < grid[new_y].len() {
                        let new_p = grid[new_y][new_x];
                        if grid[coord.1][coord.0] + 1 < new_p {
                            None
                        } else {
                            Some((new_x, new_y))
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        })
        .collect()
}

fn dijkstra(start: (usize, usize), end: (usize, usize), grid: &[Vec<isize>]) -> isize {
    let mut visited = HashSet::new();

    let mut pqueue = [start]
        .into_iter()
        .map(|s| ((0, s), s))
        .collect::<BTreeMap<_, _>>();

    let mut ans = isize::MAX;
    while let Some((c, v)) = pqueue.pop_first() {
        if visited.contains(&v) {
            continue;
        }
        visited.insert(v);
        if v == end {
            ans = c.0;
            break;
        }

        let d = c.0 + 1;
        for u in get_neighbors(v, grid) {
            pqueue.insert((d, u), u);
        }
    }
    ans
}

pub fn part1() -> isize {
    let mut end = (0, 0);
    let mut start = (0, 0);
    let grid = load_string("inputs/2022/day12.input")
        .lines()
        .enumerate()
        .map(|(y, r)| {
            r.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'E' => {
                        end = (x, y);
                        25
                    }
                    'S' => {
                        start = (x, y);
                        0
                    }
                    x => char_to_height(x),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    dijkstra(start, end, &grid)
}

pub fn part2() -> isize {
    let mut end = (0, 0);
    let mut start = vec![];
    let grid = load_string("inputs/2022/day12.input")
        .lines()
        .enumerate()
        .map(|(y, r)| {
            r.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'E' => {
                        end = (x, y);
                        25
                    }
                    'S' | 'a' => {
                        start.push((x, y));
                        0
                    }
                    x => char_to_height(x),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    start
        .into_iter()
        .map(|start| dijkstra(start, end, &grid))
        .min()
        .unwrap()
}
