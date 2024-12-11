use std::fmt::format;

use rustc_hash::FxHashMap;
use utils::bi_functors::BiFunctorExtExt;

fn solve(amount: usize) -> i64 {
    let s = include_str!("../inputs/2024/day11.input");
    let mut stones = s
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .map(|s| (s, 1))
        .collect::<FxHashMap<_, _>>();

    // println!("{stones:?}");
    for _ in 0..amount {
        let mut edit = Vec::new();
        for (k, v) in &stones {
            let k = *k;
            let v = *v;
            let f = format!("{k}");
            if k == 0 {
                edit.push((k, -v));
                edit.push((1, v));
            } else if f.len() % 2 == 0 {
                let (a, b) = f.split_at(f.len() / 2).splet(|s| s.parse::<i64>().unwrap());
                edit.push((k, -v));
                edit.push((a, v));
                edit.push((b, v));
            } else {
                edit.push((k, -v));
                edit.push((k * 2024, v));
            }
        }
        // println!("{stones:?}: {edit:?}");
        for (nk, nv) in edit {
            let entry = stones.entry(nk).or_insert(0);
            *entry += nv;
            if *entry <= 0 {
                stones.remove(&nk);
            }
        }
        // println!(
        //     "{:?}",
        //     stones
        //         .iter()
        //         .map(|(i, j)| format!("{i}").repeat(*j as usize))
        //         .collect::<Vec<_>>()
        //         .join(" ")
        // );
        // println!("After {j} blinks:\n{stones:?}");
    }

    stones.into_values().sum()
}

pub fn part1() -> i64 {
    solve(25)
}
pub fn part2() -> i64 {
    solve(75)
}
