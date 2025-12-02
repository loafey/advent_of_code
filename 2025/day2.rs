use rayon::prelude::*;
use utils::NumExt;

fn input() -> impl Iterator<Item = (u64, u64)> {
    include_str!("../inputs/2025/day2.input")
        .split(',')
        .map(|s| {
            let (a, b) = s.trim().split_once('-').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
}

// Rayon time 😎😎

pub fn part1() -> u64 {
    input()
        .par_bridge()
        .flat_map_iter(|(a, b)| {
            (a..=b).map(|i| {
                let (a, b) = i.split();
                (a == b) as u64 * i
            })
        })
        .sum()
}

pub fn part2() -> u64 {
    input()
        .par_bridge()
        .flat_map_iter(|(a, b)| {
            (a..=b).filter(|i| {
                let s = i.to_string();
                (1..=(s.len() / 2))
                    .any(|p| s.matches(&s[..p]).map(str::len).sum::<usize>() == s.len())
            })
        })
        .sum()
}
