use memoize::memoize;
use rayon::prelude::*;

fn input() -> impl Iterator<Item = Vec<u64>> {
    include_str!("../inputs/2025/day3.input")
        .lines()
        .map(|c| c.chars().map(|c| c as u64 - 0x30).collect::<Vec<_>>())
}

#[memoize]
fn solve(count: u8, data: Vec<u64>) -> u64 {
    if count == 1 {
        return data.into_iter().max().unwrap();
    }
    let mut max = 0;
    for i in 0..data.len() {
        let mut data = data.clone();

        let v = data[i];
        data = data[i + 1..].to_vec();
        if data.len() + 1 < count as usize {
            continue;
        }
        let ans = v * 10u64.pow(count as u32 - 1) + solve(count - 1, data);
        max = max.max(ans)
    }
    max
}

pub fn part1() -> u64 {
    input().par_bridge().map(|bank| solve(2, bank)).sum()
}

pub fn part2() -> u64 {
    input().par_bridge().map(|bank| solve(12, bank)).sum()
}
