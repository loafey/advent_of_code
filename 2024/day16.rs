use pathfinding::prelude::{astar_bag, dijkstra};
use rustc_hash::FxHashSet;
use utils::{
    Direction::{self, *},
    FindSome,
};

// This day was completely solved by my fantastic pathfinding lib :)

fn find(g: u8, map: Map) -> (usize, usize) {
    map.iter()
        .enumerate()
        .find_some(|(y, s)| {
            s.iter()
                .enumerate()
                .find_some(|(x, c)| if *c == g { Some((y, x)) } else { None })
        })
        .unwrap()
}

matrixy::matrixy!("../inputs/2024/day16.input");
fn successors((y, x, d): &(usize, usize, Direction)) -> Vec<((usize, usize, Direction), usize)> {
    let mut r = Vec::new();
    let (ky, kx) = match d.rotate_l() {
        Up => (y - 1, *x),
        Right => (*y, x + 1),
        Down => (y + 1, *x),
        Left => (*y, x - 1),
    };
    if MAP[ky][kx] != b'#' {
        r.push(((ky, kx, d.rotate_l()), 1001));
    }
    let (ky, kx) = match d.rotate_r() {
        Up => (y - 1, *x),
        Right => (*y, x + 1),
        Down => (y + 1, *x),
        Left => (*y, x - 1),
    };
    if MAP[ky][kx] != b'#' {
        r.push(((ky, kx, d.rotate_r()), 1001));
    }
    let (dy, dx) = match d {
        Up => (y - 1, *x),
        Right => (*y, x + 1),
        Down => (y + 1, *x),
        Left => (*y, x - 1),
    };
    if MAP[dy][dx] != b'#' {
        r.push(((dy, dx, *d), 1));
    }
    r
}

pub fn part1() -> usize {
    let (sy, sx) = find(b'S', MAP);
    let (ey, ex) = find(b'E', MAP);

    dijkstra(&(sy, sx, Right), successors, |(y, x, _)| {
        (*y, *x) == (ey, ex)
    })
    .unwrap_or_default()
    .1
}

pub fn part2() -> usize {
    let (sy, sx) = find(b'S', MAP);
    let (ey, ex) = find(b'E', MAP);

    astar_bag(
        &(sy, sx, Right),
        successors,
        |_| 0,
        |(y, x, _)| (*y, *x) == (ey, ex),
    )
    .unwrap()
    .0
    .flat_map(|v| v.into_iter().map(|(y, x, _)| (y, x)))
    .collect::<FxHashSet<_>>()
    .len()
}
