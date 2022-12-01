use std::collections::BTreeSet;

pub fn part1() -> i32 {
    include_str!("input/day1.input")
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
    let mut set = include_str!("input/day1.input")
        .split("\n\n")
        .map(|r| {
            r.split('\n')
                .filter_map(|v| v.parse::<i32>().ok())
                .sum::<i32>()
        })
        .map(|u| -1 * u)
        .collect::<BTreeSet<i32>>();
    (-1 * set.pop_first().unwrap())
        + (-1 * set.pop_first().unwrap())
        + (-1 * set.pop_first().unwrap())
}
