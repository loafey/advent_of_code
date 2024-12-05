use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use utils::bi_functors::BiFunctorExtExt;

pub fn part1() -> i64 {
    let (rules_input, input) = include_str!("../inputs/2024/day5.input")
        .split_once("\n\n")
        .unwrap();

    let mut rules: HashMap<i64, Vec<i64>> = HashMap::new();
    for r in rules_input.lines() {
        let (a, b) = r
            .split_once('|')
            .unwrap()
            .splet(|s| s.parse::<i64>().unwrap());
        rules.entry(a).or_default().push(b);
    }

    let mut sum = 0;
    'outer: for inp in input.lines().filter(|s| !s.is_empty()) {
        let mut visited = HashSet::new();
        let nums = inp
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        for num in &nums {
            if let Some(r) = rules.get(num) {
                for r in r {
                    if visited.contains(r) {
                        continue 'outer;
                    }
                }
            }
            visited.insert(num);
        }
        sum += nums[nums.len() / 2]
    }

    sum
}
pub fn part2() -> i64 {
    let (rules_input, input) = include_str!("../inputs/2024/day5.input")
        .split_once("\n\n")
        .unwrap();

    let mut rules: HashMap<i64, Vec<i64>> = HashMap::new();
    for r in rules_input.lines() {
        let (a, b) = r
            .split_once('|')
            .unwrap()
            .splet(|s| s.parse::<i64>().unwrap());
        rules.entry(a).or_default().push(b);
    }

    let mut wrongs = Vec::new();
    'outer: for inp in input.lines().filter(|s| !s.is_empty()) {
        let mut visited = HashSet::new();
        let nums = inp
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        for num in &nums {
            if let Some(r) = rules.get(num) {
                for r in r {
                    if visited.contains(r) {
                        wrongs.push(nums);
                        continue 'outer;
                    }
                }
            }
            visited.insert(num);
        }
    }

    println!("{wrongs:?}");
    wrongs.iter_mut().for_each(|v| {
        v.sort_by(|a, b| {
            if let Some(r) = rules.get(a) {
                if r.contains(b) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            } else {
                Ordering::Equal
            }
        })
    });

    let mut sum = 0;
    for wrong in wrongs {
        sum += wrong[wrong.len() / 2];
    }

    sum
}
