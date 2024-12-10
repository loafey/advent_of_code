use matrixy::matrixy;
use pathfinding::{directed::count_paths::count_paths, prelude::dfs};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rustc_hash::FxHashSet;

matrixy!("../inputs/2024/day10.input");

#[inline(always)]
fn neighbors(y: usize, x: usize) -> Vec<(usize, usize)> {
    let cur = MAP[y][x];
    let mut res = Vec::new();
    macro_rules! check {
        ($ymod:expr, $xmod:expr) => {
            if let Some(c) = MAP.get($ymod).and_then(|r| r.get($xmod)) {
                if (c.wrapping_sub(cur)) == 1 {
                    res.push(($ymod, $xmod));
                }
            }
        };
    }
    check!(y.wrapping_sub(1), x);
    check!(y.wrapping_add(1), x);
    check!(y, x.wrapping_sub(1));
    check!(y, x.wrapping_add(1));
    res
}

fn solve(func: fn((usize, usize)) -> usize) -> usize {
    let zeros = MAP
        .iter()
        .enumerate()
        .flat_map(|(y, r)| {
            r.iter()
                .enumerate()
                .filter(|(_, c)| **c == b'0')
                .map(move |(x, _)| (y, x))
        })
        .collect::<Vec<_>>();

    zeros.into_par_iter().map(func).sum()
}

pub fn part1() -> usize {
    solve(|p| {
        let mut ignore = FxHashSet::default();
        let mut local = 0;
        while let Some(pos) = dfs(
            p,
            |(y, x)| neighbors(*y, *x),
            |(y, x)| MAP[*y][*x] == b'9' && !ignore.contains(&(*y, *x)),
        ) {
            ignore.insert(pos.last().copied().unwrap());
            local += 1;
        }

        local
    })
}
pub fn part2() -> usize {
    solve(|p| count_paths(p, |(y, x)| neighbors(*y, *x), |(y, x)| MAP[*y][*x] == b'9'))
}
