use std::{collections::BTreeSet, ops::RangeInclusive};

fn input() -> (Vec<RangeInclusive<u64>>, impl Iterator<Item = u64>) {
    let (top, bot) = include_str!("../inputs/2025/day5.input")
        .split_once("\n\n")
        .unwrap();
    let top = top
        .lines()
        .map(|s| {
            let (a, b) = s.split_once("-").unwrap();
            a.parse::<u64>().unwrap()..=b.parse::<u64>().unwrap()
        })
        .collect();
    let bot = bot.lines().map(|s| s.parse::<u64>().unwrap());
    (top, bot)
}

pub fn part1() -> u64 {
    let (mut ranges, ingd) = input();
    ranges.sort_by_key(|r| *r.start());
    let mut ok = 0;
    let mut ingd = ingd.collect::<Vec<u64>>();
    ingd.sort();
    for i in ingd {
        let mut to_remove = Vec::new();
        for (ri, r) in ranges.iter().enumerate() {
            if r.contains(&i) {
                ok += 1;
                break;
            }
            if *r.end() < i {
                to_remove.push(ri);
            }
        }
        for (o, r) in to_remove.into_iter().enumerate() {
            ranges.remove(r - o);
        }
    }
    ok
}

pub fn part2() -> u64 {
    let (mut ranges, _) = input();
    let mut diff = 0;
    ranges.sort_by_key(|r| *r.start());
    let mut last_end = u64::MAX;
    for r in ranges {
        let start = if *r.start() <= last_end && last_end != u64::MAX {
            last_end + 1
        } else {
            *r.start()
        };
        if *r.end() <= last_end && last_end != u64::MAX {
            continue;
        }
        diff += (start..=*r.end()).count() as u64;
        last_end = *r.end();
    }
    diff
}
