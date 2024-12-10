use pathfinding::{directed::count_paths::count_paths, prelude::dfs};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rustc_hash::FxHashSet;

// I know this is stupid :)) let me cook
static mut MAP: Vec<Vec<i8>> = Vec::new();
macro_rules! map {
    () => {
        #[allow(unused_unsafe)]
        unsafe {
            #[allow(static_mut_refs)]
            &MAP
        }
    };
}

#[inline(always)]
fn neighbors(y: usize, x: usize) -> Vec<(usize, usize)> {
    let cur = map!()[y][x];
    let mut res = Vec::new();
    macro_rules! check {
        ($ymod:expr, $xmod:expr) => {
            if let Some(c) = map!().get($ymod).and_then(|r| r.get($xmod)) {
                if (c - cur) == 1 {
                    res.push(($ymod, $xmod));
                }
            }
        };
    }
    check!(y - 1, x);
    check!(y + 1, x);
    check!(y, x - 1);
    check!(y, x + 1);
    res
}

fn solve(func: fn((usize, usize)) -> usize) -> usize {
    unsafe {
        if map!().is_empty() {
            MAP = include_str!("../inputs/2024/day10.input")
                .lines()
                .filter(|s| !s.is_empty())
                .map(|s| s.bytes().map(|c| (c - 0x30) as i8).collect::<Vec<_>>())
                .collect::<Vec<_>>();
        }
    };
    let zeros = map!()
        .iter()
        .enumerate()
        .flat_map(|(y, r)| {
            r.iter()
                .enumerate()
                .filter(|(_, c)| **c == 0)
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
            |(y, x)| map!()[*y][*x] == 9 && !ignore.contains(&(*y, *x)),
        ) {
            ignore.insert(pos.last().copied().unwrap());
            local += 1;
        }

        local
    })
}
pub fn part2() -> usize {
    solve(|p| count_paths(p, |(y, x)| neighbors(*y, *x), |(y, x)| map!()[*y][*x] == 9))
}
