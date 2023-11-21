use crate::utils::load_string;
use std::collections::BTreeSet;

pub fn part1() -> i32 {
    load_string("inputs/2022/day1.input")
        .split("\n\n")
        .map(|r| {
            r.split('\n')
                .filter_map(|r| r.parse::<i32>().ok())
                .sum::<i32>()
        })
        .max()
        .unwrap_or_default()
}

pub fn part2() -> i32 {
    let mut set = load_string("inputs/2022/day1.input")
        .split("\n\n")
        .map(|r| {
            r.split('\n')
                .filter_map(|v| v.parse::<i32>().ok())
                .sum::<i32>()
        })
        .map(|u| -u)
        .collect::<BTreeSet<i32>>();
    (-set.pop_first().unwrap()) + (-set.pop_first().unwrap()) + (-set.pop_first().unwrap())
}
