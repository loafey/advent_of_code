use pathfinding::prelude::{dijkstra, yen};
use rustc_hash::FxHashSet;
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
    let mut r = vec![
        ((*y, *x, d.rotate_l()), 1000),
        ((*y, *x, d.rotate_r()), 1000),
    ];
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

    dijkstra(&(sy, sx, East), successors, |(y, x, _)| {
        (*y, *x) == (ey, ex)
    })
    .unwrap_or_default()
    .1
}
pub fn part2() -> usize {
    let (sy, sx) = find(b'S', MAP);
    let (ey, ex) = find(b'E', MAP);

    let mut paths = yen(
        &(sy, sx, East),
        successors,
        |(y, x, _)| (*y, *x) == (ey, ex),
        15, // My input only has 15 shortest paths
    );
    paths.sort_by_key(|(_, s)| *s);
    let k = paths[0].1;
    paths.retain(|(_, s)| *s == k);

    paths
        .into_iter()
        .flat_map(|(v, _)| v.into_iter().map(|(y, x, _)| (y, x)))
        .collect::<FxHashSet<_>>()
        .len()
}
