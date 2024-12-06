use std::collections::HashSet;

use utils::FindSome;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
#[repr(u8)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

pub fn part1() -> usize {
    let m = include_str!("../inputs/2024/day6.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (mut y, mut x) = m
        .iter()
        .enumerate()
        .find_some(|(y, v)| {
            v.iter().enumerate().find_some(|(x, a)| {
                if *a == '^' {
                    Some((y as isize, x as isize))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let mut visited = HashSet::new();
    let mut dir = Dir::Up;
    loop {
        visited.insert((y, x));
        let (ny, nx) = match dir {
            Dir::Up => (y - 1, x),
            Dir::Down => (y + 1, x),
            Dir::Left => (y, x - 1),
            Dir::Right => (y, x + 1),
        };
        let Some(c) = m.get(ny as usize).and_then(|v| v.get(nx as usize)) else {
            break;
        };
        if *c == '#' {
            dir = match (dir as u8 + 1) % 4 {
                0 => Dir::Up,
                1 => Dir::Right,
                2 => Dir::Down,
                3 => Dir::Left,
                _ => panic!(),
            };
        } else {
            y = ny;
            x = nx;
        }
    }

    visited.len()
}
pub fn part2() -> usize {
    let m = include_str!("../inputs/2024/day6.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (y, x) = m
        .iter()
        .enumerate()
        .find_some(|(y, v)| {
            v.iter().enumerate().find_some(|(x, a)| {
                if *a == '^' {
                    Some((y as isize, x as isize))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let mut loopy = 0;
    for py in 0..m.len() {
        for px in 0..m[py].len() {
            let mut visited = HashSet::new();
            let mut dir = Dir::Up;
            let mut y = y;
            let mut x = x;
            loop {
                if !visited.insert((y, x, dir)) {
                    loopy += 1;
                    break;
                }
                let (ny, nx) = match dir {
                    Dir::Up => (y - 1, x),
                    Dir::Right => (y, x + 1),
                    Dir::Down => (y + 1, x),
                    Dir::Left => (y, x - 1),
                };
                let Some(c) = m.get(ny as usize).and_then(|v| v.get(nx as usize)) else {
                    break;
                };
                if *c == '#' || (ny, nx) == (py as isize, px as isize) {
                    dir = match (dir as u8 + 1) % 4 {
                        0 => Dir::Up,
                        1 => Dir::Right,
                        2 => Dir::Down,
                        3 => Dir::Left,
                        _ => panic!(),
                    };
                } else {
                    y = ny;
                    x = nx;
                }
            }
        }
    }
    loopy
}
