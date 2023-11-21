use crate::utils::load_string;
use std::collections::HashSet;

pub fn part1() -> i32 {
    let input = load_string("inputs/2018/day1.input");
    input
        .split('\n')
        .map(|s| {
            let op = &s[0..1];
            s[1..].parse::<i32>().unwrap() * if op == "-" { -1 } else { 1 }
        })
        .sum()
}
pub fn part2() -> i32 {
    let input = load_string("inputs/2018/day1.input");
    let mut freq = 0;
    let mut list = HashSet::new();
    list.insert(freq);

    let parsed = input
        .split('\n')
        .map(|s| {
            let op = &s[0..1];
            s[1..].parse::<i32>().unwrap() * if op == "-" { -1 } else { 1 }
        })
        .cycle();

    for s in parsed {
        freq += s;
        if !list.contains(&freq) {
            list.insert(freq);
        } else {
            break;
        }
    }

    freq
}
