use std::collections::{BTreeMap, BTreeSet};

pub fn part1() -> usize {
    let input = include_str!("../../inputs/2020/day6.input");
    let mut res = 0;
    for group in input.split("\n\n") {
        let mut set = BTreeSet::new();
        for line in group.lines() {
            for c in line.chars() {
                set.insert(c);
            }
        }
        res += set.len();
    }

    res
}

pub fn part2() -> i64 {
    let input = include_str!("../../inputs/2020/day6.input");

    let mut res = 0;
    for group in input.split("\n\n") {
        let s = group.lines().count();
        let mut pairs = BTreeMap::new();
        for line in group.lines() {
            for c in line.chars() {
                *pairs.entry(c).or_insert(0) += 1;
            }
        }
        for (_, count) in pairs {
            if count == s {
                res += 1;
            }
        }
    }

    res
}
