use rustc_hash::FxHashMap;
use utils::{bi_functors::BiFunctorExtExt, NumExt};

fn solve(amount: usize) -> i64 {
    let s = include_str!("../inputs/2024/day11.input");
    let mut stones = s
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .map(|s| (s, 1))
        .collect::<FxHashMap<_, _>>();

    for _ in 0..amount {
        let mut edit = Vec::new();
        for (k, v) in &stones {
            let k = *k;
            let v = *v;
            if k == 0 {
                edit.push((k, -v));
                edit.push((1, v));
            } else if k.len() % 2 == 0 {
                let (a, b) = k.split();
                edit.push((k, -v));
                edit.push((a, v));
                edit.push((b, v));
            } else {
                edit.push((k, -v));
                edit.push((k * 2024, v));
            }
        }
        for (nk, nv) in edit {
            let entry = stones.entry(nk).or_insert(0);
            *entry += nv;
            if *entry <= 0 {
                stones.remove(&nk);
            }
        }
    }

    stones.into_values().sum()
}

pub fn part1() -> i64 {
    solve(25)
}
pub fn part2() -> i64 {
    solve(75)
}
