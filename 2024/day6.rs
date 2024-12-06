use rustc_hash::FxHashSet;
use std::mem::transmute;
use utils::FindSome;
type Grid = &'static [[u8; 131]; 130];

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

fn find_start(m: Grid) -> (isize, isize) {
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

fn get_path(mut y: isize, mut x: isize, m: Grid) -> FxHashSet<(isize, isize)> {
    let mut visited = FxHashSet::default();
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
        if *c == b'#' {
            dir = dir.inc();
        } else {
            y = ny;
            x = nx;
        }
    }
    visited
}
pub fn part1() -> usize {
    let (m, _) =
        unsafe { transmute::<&str, (Grid, usize)>(include_str!("../inputs/2024/day6.input")) };
    let (y, x) = find_start(m);
    get_path(y, x, m).len()
}
pub fn part2() -> usize {
    let (m, _) =
        unsafe { transmute::<&str, (Grid, usize)>(include_str!("../inputs/2024/day6.input")) };
    let (y, x) = find_start(m);
    let og_path = get_path(y, x, m);

    let mut loopy = 0;
    let mut visited = FxHashSet::default();
    for (py, px) in og_path {
        visited.clear();
        let mut dir = Dir::Up;
        let mut y = y;
        let mut x = x;
        loopy += loop {
            if !visited.insert((y, x, dir)) {
                break 1;
            }
            let (ny, nx) = match dir {
                Dir::Up => (y - 1, x),
                Dir::Right => (y, x + 1),
                Dir::Down => (y + 1, x),
                Dir::Left => (y, x - 1),
            };
            let Some(c) = m.get(ny as usize).and_then(|v| v.get(nx as usize)) else {
                break 0;
            };
            if *c == b'#' || (ny, nx) == (py, px) {
                dir = dir.inc();
            } else {
                y = ny;
                x = nx;
            }
        }
    }

    loopy
}
