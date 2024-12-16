use std::iter::Successors;

use pathfinding::prelude::{dijkstra, yen};
use rustc_hash::{FxHashMap, FxHashSet};
use utils::{Direction, Direction::*, FindSome};

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
        North => (y - 1, *x),
        East => (*y, x + 1),
        South => (y + 1, *x),
        West => (*y, x - 1),
    };
    if MAP[ky][kx] != b'#' {
        r.push(((ky, kx, d.rotate_l()), 1001));
    }
    let (ky, kx) = match d.rotate_r() {
        North => (y - 1, *x),
        East => (*y, x + 1),
        South => (y + 1, *x),
        West => (*y, x - 1),
    };
    if MAP[ky][kx] != b'#' {
        r.push(((ky, kx, d.rotate_r()), 1001));
    }
    let (dy, dx) = match d {
        North => (y - 1, *x),
        East => (*y, x + 1),
        South => (y + 1, *x),
        West => (*y, x - 1),
    };
    if MAP[dy][dx] != b'#' {
        r.push(((dy, dx, *d), 1));
    }
    r
}

pub fn part1() -> usize {
    let (sy, sx) = find(b'S', MAP);
    let (ey, ex) = find(b'E', MAP);

    // let map = expr();

    dijkstra(
        &(sy, sx, East),
        successors, //|c| map.get(c).unwrap_or(&Vec::new()).clone(),
        |(y, x, _)| (*y, *x) == (ey, ex),
    )
    .unwrap_or_default()
    .1
}

#[allow(clippy::type_complexity)]
fn expr() -> FxHashMap<(usize, usize, Direction), Vec<((usize, usize, Direction), i32)>> {
    let (sy, sx) = find(b'S', MAP);
    let (ey, ex) = find(b'E', MAP);
    let mut map = FxHashMap::default();
    let mut visited = FxHashSet::default();
    let mut stack = vec![(sy, sx, East)];
    while let Some((y, x, d)) = stack.pop() {
        if !visited.insert((y, x, d)) {
            continue;
        }

        let mut r = Vec::new();
        let (dy, dx) = match d {
            North => (y - 1, x),
            East => (y, x + 1),
            South => (y + 1, x),
            West => (y, x - 1),
        };
        if MAP[dy][dx] != b'#' {
            r.push(((dy, dx, d), 1));
            stack.push((dy, dx, d));
        }
        let (ky, kx) = match d.rotate_r() {
            North => (y - 1, x),
            East => (y, x + 1),
            South => (y + 1, x),
            West => (y, x - 1),
        };
        if MAP[ky][kx] != b'#' {
            r.push(((y, x, d.rotate_r()), 1000));
            stack.push((y, x, d.rotate_r()));
        }
        let (ky, kx) = match d.rotate_l() {
            North => (y - 1, x),
            East => (y, x + 1),
            South => (y + 1, x),
            West => (y, x - 1),
        };
        if MAP[ky][kx] != b'#' {
            r.push(((y, x, d.rotate_l()), 1000));
            stack.push((y, x, d.rotate_l()));
        }

        map.insert((y, x, d), r);
    }

    let mut last = 0;
    loop {
        let to_remove = map
            .iter()
            .filter(|(m, v)| v.len() == 1 && !(m.0 == ey && m.1 == ex))
            .map(|(k, v)| (*k, v[0]))
            .collect::<Vec<_>>();
        for (k, (d1, w1)) in &to_remove {
            for n in map.values_mut() {
                for (d2, w2) in n {
                    // println!("{k:?} == {d2:?}");
                    if *d2 == *k {
                        *d2 = *d1;
                        *w2 += w1;
                        break;
                    }
                }
            }
        }
        for (k, _) in to_remove {
            map.remove(&k);
        }

        let to_remove = map
            .iter()
            .filter(|(_, v)| v.is_empty())
            .map(|(k, _)| *k)
            .collect::<Vec<_>>();
        for k in &to_remove {
            for n in map.values_mut() {
                if let Some((i, _)) = n.iter().enumerate().find(|(_, (k2, _))| *k == *k2) {
                    n.remove(i);
                }
            }
        }
        for k in to_remove {
            map.remove(&k);
        }

        if last == map.len() {
            break;
        }
        last = map.len();
    }

    // for (v, d) in &map {
    //     println!("{v:?}:\t{d:?}")
    // }

    // let mut draw = MAP
    //     .iter()
    //     .map(|r| {
    //         r.iter()
    //             .filter(|c| **c != b'\n')
    //             .map(|c| *c as char)
    //             .map(|c| if c == '.' { ' ' } else { c })
    //             .collect::<Vec<_>>()
    //     })
    //     .collect::<Vec<_>>();
    // for (k, _) in map {
    //     let (y, x, _) = k;
    //     draw[y][x] = '*';
    // }
    // for r in draw {
    //     for c in r {
    //         print!("{c}")
    //     }
    //     println!()
    // }

    map
}

pub fn part2() -> usize {
    expr();

    let (sy, sx) = find(b'S', MAP);
    let (ey, ex) = find(b'E', MAP);

    let mut paths = yen(
        &(sy, sx, East),
        successors,
        |(y, x, _)| (*y, *x) == (ey, ex),
        18,
    );
    paths.sort_by_key(|(_, s)| *s);
    let k = paths[0].1;
    paths.retain(|(_, s)| *s == k);

    // let mut map = MAP
    //     .iter()
    //     .map(|v| {
    //         v.iter()
    //             .map(|c| *c as char)
    //             .filter(|c| *c != '\n')
    //             .collect::<Vec<_>>()
    //     })
    //     .collect::<Vec<_>>();
    // for (p, _) in &paths {
    //     for (y, x, _) in p {
    //         map[*y][*x] = 'O';
    //     }
    // }
    // for r in map {
    //     for c in r {
    //         print!("{c}");
    //     }
    //     println!()
    // }

    paths
        .into_iter()
        .flat_map(|(v, _)| v.into_iter().map(|(y, x, _)| (y, x)))
        .collect::<FxHashSet<_>>()
        .len()
}
