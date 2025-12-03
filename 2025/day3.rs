use rayon::prelude::*;
use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    sync::{LazyLock, RwLock},
};

fn input() -> impl Iterator<Item = Vec<u64>> {
    include_str!("../inputs/2025/day3.input")
        .lines()
        .map(|c| c.chars().map(|c| c as u64 - 0x30).collect::<Vec<_>>())
}

static CACHE: LazyLock<RwLock<HashMap<u64, u64>>> = LazyLock::new(Default::default);

fn solve(count: u8, data: &[u64]) -> u64 {
    let mut hasher = DefaultHasher::new();
    count.hash(&mut hasher);
    data.hash(&mut hasher);
    if let Some(cache) = CACHE.read().unwrap().get(&hasher.finish()) {
        return *cache;
    }

    if count == 1 {
        return *data.iter().max().unwrap();
    }
    let mut max = 0;
    for i in 0..data.len() {
        let mut data = data;

        let v = data[i];
        data = &data[i + 1..];
        if data.len() + 1 < count as usize {
            continue;
        }
        let ans = v * 10u64.pow(count as u32 - 1) + solve(count - 1, data);
        max = max.max(ans)
    }

    let mut hasher = DefaultHasher::new();
    count.hash(&mut hasher);
    data.hash(&mut hasher);
    CACHE.write().unwrap().insert(hasher.finish(), max);
    max
}

pub fn part1() -> u64 {
    input().par_bridge().map(|bank| solve(2, &bank[..])).sum()
}

pub fn part2() -> u64 {
    input().par_bridge().map(|bank| solve(12, &bank[..])).sum()
}
