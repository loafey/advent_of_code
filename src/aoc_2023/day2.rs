use std::collections::HashMap;

use crate::utils::load_string;

type Input<'l> = Vec<(usize, Vec<Vec<(usize, &'l str)>>)>;
fn input(s: &str) -> Input<'_> {
    s.lines()
        .map(|row| {
            let (g, s) = row.split_once(':').unwrap();
            let index = g.split_once(' ').unwrap().1.parse::<usize>().unwrap();
            let sets = s
                .split(';')
                .map(|s| {
                    s.split(',')
                        .map(|s| {
                            let (amount, color) = s.trim().split_once(' ').unwrap();
                            (amount.parse::<usize>().unwrap(), color)
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            (index, sets)
        })
        .collect::<Vec<_>>()
}

pub fn part1() -> usize {
    let s = load_string("inputs/2023/day2.input");
    let inputs = input(&s);
    let inventory: HashMap<_, _> = [("red", 12), ("green", 13), ("blue", 14)].into();
    inputs
        .into_iter()
        .filter_map(|(index, sets)| {
            let mut valid = true;

            'outer: for set in sets {
                let mut inventory = inventory.clone();
                for (amount, item) in set {
                    if let Some(count) = inventory.get_mut(item) {
                        if *count >= amount {
                            *count -= amount;
                            if *count == 0 {
                                inventory.remove(item);
                            }
                        } else {
                            valid = false;
                            break 'outer;
                        }
                    } else {
                        valid = false;
                    }
                }
            }
            valid.then_some(index)
        })
        .sum()
}
pub fn part2() -> usize {
    let s = load_string("inputs/2023/day2.input");
    let inputs = input(&s);
    inputs
        .into_iter()
        .map(|(_, sets)| {
            let mut nums: HashMap<_, _> = [("red", 0), ("green", 0), ("blue", 0)].into();
            for set in sets {
                for (amount, item) in set {
                    if amount > nums[item] {
                        nums.insert(item, amount);
                    }
                }
            }
            nums.values().copied().product::<usize>()
        })
        .sum()
}
