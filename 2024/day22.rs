use rayon::prelude::*;
use rustc_hash::FxHashMap;
use std::collections::{BTreeMap, VecDeque};

trait Nummy {
    fn mix(self, rhs: Self) -> Self;
    fn prune(self) -> Self;
}
impl Nummy for i64 {
    fn mix(self, rhs: Self) -> Self {
        rhs ^ self
    }

    fn prune(self) -> Self {
        self % 16777216
    }
}

pub fn part1() -> i64 {
    let codes = include_str!("../inputs/2024/day22.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap());
    let mut sum = 0;
    for mut code in codes {
        for _ in 0..2000 {
            code = math(code).0;
        }
        sum += code;
    }
    sum
}

fn math(code: i64) -> (i64, i64, i64) {
    let p1 = code.mix(code * 64).prune();
    let p2 = p1.mix(p1 / 32).prune();
    let p3 = p2.mix(p2 * 2048).prune();
    let price = p3 % 10;
    let change = (p3 % 10) - (code % 10);
    (p3, price, change)
}

pub fn part2() -> i64 {
    let codes = include_str!("../inputs/2024/day22.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut lookup: BTreeMap<VecDeque<i64>, BTreeMap<i64, i64>> = BTreeMap::default();
    for code in &codes {
        let og_code = *code;
        let mut code = *code;
        let mut seq = VecDeque::new();
        for _ in 0..2000 {
            let (nc, price, change) = math(code);
            seq.push_back(change);
            if seq.len() > 4 {
                seq.pop_front();
                lookup
                    .entry(seq.clone())
                    .or_default()
                    .entry(og_code)
                    .or_insert(price);
            }
            code = nc;
        }
    }
    lookup
        .into_values()
        .map(|v| v.into_values().sum())
        .max()
        .unwrap_or_default()
}
