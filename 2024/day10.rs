use pathfinding::{
    directed,
    prelude::{bfs, dfs},
};
use rustc_hash::FxHashSet;

pub fn part1() -> i64 {
    let map = include_str!("../inputs/2024/day10.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.bytes().map(|c| (c - 0x30) as i8).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let zeros = map
        .iter()
        .enumerate()
        .flat_map(|(y, r)| {
            r.iter()
                .enumerate()
                .filter(|(_, c)| **c == 0)
                .map(move |(x, _)| (y, x))
        })
        .collect::<Vec<_>>();

    let mut res = 0;
    for p in zeros {
        let mut ignore = FxHashSet::default();
        let mut local = 0;
        while let Some(pos) = dfs(
            p,
            |(y, x)| {
                // println!("Looking at {y}:{x}");
                let cur = map[*y][*x];
                let mut res = Vec::new();
                if let Some(c) = map.get(y - 1).and_then(|r| r.get(*x)) {
                    if (c - cur) == 1 {
                        res.push((y - 1, *x));
                    }
                }
                if let Some(c) = map.get(y + 1).and_then(|r| r.get(*x)) {
                    if (c - cur) == 1 {
                        res.push((y + 1, *x));
                    }
                }
                if let Some(c) = map.get(*y).and_then(|r| r.get(x - 1)) {
                    if (c - cur) == 1 {
                        res.push((*y, x - 1));
                    }
                }
                if let Some(c) = map.get(*y).and_then(|r| r.get(x + 1)) {
                    if (c - cur) == 1 {
                        res.push((*y, x + 1));
                    }
                }
                res
            },
            |(y, x)| map[*y][*x] == 9 && !ignore.contains(&(*y, *x)),
        ) {
            ignore.insert(pos.last().copied().unwrap());
            local += 1;
        }

        res += local;
    }

    res
}
pub fn part2() -> usize {
    let map = include_str!("../inputs/2024/day10.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.bytes().map(|c| (c - 0x30) as i8).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let zeros = map
        .iter()
        .enumerate()
        .flat_map(|(y, r)| {
            r.iter()
                .enumerate()
                .filter(|(_, c)| **c == 0)
                .map(move |(x, _)| (y, x))
        })
        .collect::<Vec<_>>();

    let mut res = 0;
    for p in zeros {
        let ans = directed::count_paths::count_paths(
            p,
            |(y, x)| {
                let cur = map[*y][*x];
                let mut res = Vec::new();
                macro_rules! check {
                    ($ymod:expr, $xmod:expr) => {
                        if let Some(c) = map.get($ymod).and_then(|r| r.get($xmod)) {
                            if (c - cur) == 1 {
                                res.push(($ymod, $xmod));
                            }
                        }
                    };
                }
                check!(y - 1, *x);
                check!(y + 1, *x);
                check!(*y, x - 1);
                check!(*y, x + 1);
                res
            },
            |(y, x)| map[*y][*x] == 9,
        );

        res += ans
    }

    res
}
