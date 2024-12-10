use rayon::prelude::*;
use rustc_hash::{FxBuildHasher, FxHashSet as HashSet};
use std::hash::Hash;
use utils::FindSome;

matrixy::matrixy!("../inputs/2024/day6.input");

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

fn find_start(m: Map) -> (isize, isize) {
    m.iter()
        .enumerate()
        .find_some(|(y, v)| {
            v.iter().enumerate().find_some(|(x, a)| {
                if *a == b'^' {
                    Some((y as isize, x as isize))
                } else {
                    None
                }
            })
        })
        .unwrap()
}

#[derive(Eq)]
struct Visited {
    x: isize,
    y: isize,
    dir: Dir,
}
impl Hash for Visited {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
impl PartialEq for Visited {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn get_path(mut y: isize, mut x: isize, m: Map) -> HashSet<Visited> {
    let mut visited = HashSet::with_capacity_and_hasher(5208, FxBuildHasher);
    let mut dir = Dir::Up;
    loop {
        visited.insert(Visited { x, y, dir });
        let (ny, nx) = match dir {
            Dir::Up => (y - 1, x),
            Dir::Down => (y + 1, x),
            Dir::Left => (y, x - 1),
            Dir::Right => (y, x + 1),
        };
        let Some(c) = m.get(ny as usize).and_then(|v| v.get(nx as usize)) else {
            break;
        };
        if *c == b'#' {
            dir = dir.inc();
        } else {
            y = ny;
            x = nx;
        }
    }
    visited.remove(&Visited { x, y, dir: Dir::Up });
    visited
}
pub fn part1() -> usize {
    let (y, x) = find_start(MAP);
    get_path(y, x, MAP).len()
}
pub fn part2() -> usize {
    let (y, x) = find_start(MAP);
    let og_path = get_path(y, x, MAP);

    og_path
        .into_par_iter()
        .map(
            |Visited {
                 x: px,
                 y: py,
                 mut dir,
             }| {
                let mut visited = HashSet::with_capacity_and_hasher(6058, FxBuildHasher);
                let mut y = py
                    + match dir {
                        Dir::Up => 1,
                        Dir::Down => -1,
                        _ => 0,
                    };
                let mut x = px
                    + match dir {
                        Dir::Right => -1,
                        Dir::Left => 1,
                        _ => 0,
                    };
                loop {
                    if !visited.insert((y, x, dir)) {
                        break 1;
                    }
                    let (ny, nx) = match dir {
                        Dir::Up => (y - 1, x),
                        Dir::Right => (y, x + 1),
                        Dir::Down => (y + 1, x),
                        Dir::Left => (y, x - 1),
                    };
                    let Some(c) = MAP.get(ny as usize).and_then(|v| v.get(nx as usize)) else {
                        break 0;
                    };
                    if *c == b'#' || (ny, nx) == (py, px) {
                        dir = dir.inc();
                    } else {
                        y = ny;
                        x = nx;
                    }
                }
            },
        )
        .sum()
}
