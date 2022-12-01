use std::collections::BTreeSet;

pub fn part1() -> usize {
    include_str!("input/day1.input")
        .split("\n\n")
        .map(|r| {
            r.split('\n')
                .filter_map(|r| r.parse::<usize>().ok())
                .sum::<usize>()
        })
        .max()
        .unwrap_or_default()
}

pub fn part2() -> Option<usize> {
    let mut set = include_str!("input/day1.input")
        .split("\n\n")
        .map(|r| {
            r.split('\n')
                .filter_map(|r| r.parse::<usize>().ok())
                .sum::<usize>()
        })
        .collect::<BTreeSet<usize>>();
    set.pop_last().unwrap_or_default()
        + set.pop_last().unwrap_or_default()
        + set.pop_last().unwrap_or_default()
}
