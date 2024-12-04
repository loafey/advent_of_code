use std::collections::{BinaryHeap, HashMap};

use utils::parse_next;

pub fn part1() -> i64 {
    let s = include_str!("../inputs/2024/day1.input");
    let mut l = BinaryHeap::new();
    let mut r = BinaryHeap::new();

    for i in s.lines().filter(|s| !s.is_empty()) {
        let mut nums = i.split_whitespace();
        l.push(parse_next::<i64>(&mut nums));
        r.push(parse_next::<i64>(&mut nums));
    }

    l.into_iter_sorted()
        .zip(r.into_iter_sorted())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

pub fn part2() -> i64 {
    let s = include_str!("../inputs/2024/day1.input");
    let mut left = Vec::new();
    let mut right: HashMap<i64, i64> = HashMap::new();

    for i in s.lines().filter(|s| !s.is_empty()) {
        let mut nums = i.split_whitespace();
        let value = parse_next(&mut nums);
        let key = parse_next(&mut nums);

        left.push(value);
        *right.entry(key).or_default() += 1;
    }

    left.into_iter()
        .map(|v| v * right.get(&v).copied().unwrap_or_default())
        .sum()
}
