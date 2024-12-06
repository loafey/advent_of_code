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
impl Dir {
    pub fn inc(self) -> Self {
        match (self as u8 + 1) % 4 {
            0 => Dir::Up,
            1 => Dir::Right,
            2 => Dir::Down,
            3 => Dir::Left,
            _ => panic!(),
        }
    }
}

pub fn get_path(mut y: isize, mut x: isize, m: &[Vec<char>]) -> HashSet<(isize, isize)> {
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
            dir = dir.inc();
        } else {
            y = ny;
            x = nx;
        }
    }
    visited
}

pub fn part1() -> usize {
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

    get_path(y, x, &m).len()
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

    let og_path = get_path(y, x, &m);

    let mut loopy = 0;
    for (py, px) in og_path {
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
            if *c == '#' || (ny, nx) == (py, px) {
                dir = dir.inc();
            } else {
                y = ny;
                x = nx;
            }
        }
    }

    loopy
}
