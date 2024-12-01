use std::collections::BTreeMap;

pub fn part1() -> i64 {
    let s = include_str!("../inputs/2024/day1.input");
    let mut left = Vec::new();
    let mut right = Vec::new();

    for i in s.lines().filter(|s| !s.is_empty()) {
        let mut nums = i.split_whitespace();
        left.push(nums.next().unwrap().parse::<i64>().unwrap());
        right.push(nums.next().unwrap().parse::<i64>().unwrap());
    }
    left.sort();
    left.reverse();
    right.sort();
    right.reverse();

    let mut ans = 0;
    while let Some(left) = left.pop() {
        let right = right.pop().unwrap();
        ans += (left - right).abs();
    }

    ans
}

pub fn part2() -> i64 {
    let s = include_str!("../inputs/2024/day1.input");
    let mut left = Vec::new();
    let mut right: BTreeMap<i64, i64> = BTreeMap::new();

    for i in s.lines().filter(|s| !s.is_empty()) {
        let mut nums = i.split_whitespace();
        let value = nums.next().unwrap().parse::<i64>().unwrap();
        left.push(value);
        let key = nums.next().unwrap().parse::<i64>().unwrap();
        *right.entry(key).or_default() += 1;
    }

    let mut ans = 0;
    for v in left {
        ans += v * right.get(&v).copied().unwrap_or_default();
    }

    ans
}
