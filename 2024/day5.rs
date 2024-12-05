use arrayvec::ArrayVec;
use micromap::Set;
use std::{cmp::Ordering::*, collections::HashMap};
use utils::bi_functors::BiFunctorExtExt;

fn input() -> (HashMap<i64, Set<i64, 25>>, &'static str) {
    let (rules_input, input) = include_str!("../inputs/2024/day5.input")
        .split_once("\n\n")
        .unwrap();

    let mut rules: HashMap<i64, Set<i64, 25>> = HashMap::new();
    for r in rules_input.lines() {
        let (a, b) = r
            .split_once('|')
            .unwrap()
            .splet(|s| s.parse::<i64>().unwrap());
        rules.entry(a).or_default().insert(b);
    }
    (rules, input)
}

pub fn part1() -> i64 {
    let (rules, input) = input();
    let mut sum = 0;
    'outer: for inp in input.lines().filter(|s| !s.is_empty()) {
        let mut visited: Set<_, 25> = Set::new();
        let nums = inp
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<ArrayVec<_, 23>>();

        for num in &nums {
            if let Some(r) = rules.get(num) {
                for r in r {
                    if visited.contains_key(r) {
                        continue 'outer;
                    }
                }
            }
            visited.insert(*num);
        }
        sum += nums[nums.len() / 2]
    }

    sum
}
pub fn part2() -> i64 {
    let (rules, input) = input();
    let mut sum = 0;
    'outer: for inp in input.lines().filter(|s| !s.is_empty()) {
        let mut visited: Set<_, 25> = Set::new();
        let nums = inp
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        for num in &nums {
            if let Some(r) = rules.get(num) {
                for r in r {
                    if visited.contains_key(r) {
                        let mut nums = nums;
                        nums.sort_by(|a, b| match rules.get(a) {
                            Some(r) => match r.contains_key(b) {
                                true => Less,
                                false => Greater,
                            },
                            None => Equal,
                        });
                        sum += nums[nums.len() / 2];

                        continue 'outer;
                    }
                }
            }
            visited.insert(*num);
        }
    }

    sum
}
