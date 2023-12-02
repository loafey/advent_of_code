use crate::utils::load_string;
use std::{collections::HashMap, hint::unreachable_unchecked};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}
use Color::*;
impl<'l> From<&'l str> for Color {
    fn from(value: &'l str) -> Self {
        match value {
            "red" => Red,
            "green" => Green,
            "blue" => Blue,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

type Input = Vec<(usize, Vec<Vec<(Color, usize)>>)>;
fn input(s: &str) -> Input {
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
                            (color.into(), amount.parse::<usize>().unwrap())
                        })
                        .collect()
                })
                .collect();
            (index, sets)
        })
        .collect()
}

pub fn part1() -> usize {
    let inventory = HashMap::from([(Red, 12), (Green, 13), (Blue, 14)]);
    input(&load_string("inputs/2023/day2.input"))
        .into_iter()
        .filter_map(|(index, sets)| {
            let mut valid = true;

            'outer: for set in sets {
                let mut inventory = inventory.clone();
                for (item, amount) in set {
                    if let Some(count) = inventory.get_mut(&item) {
                        if *count >= amount {
                            *count -= amount;
                            if *count == 0 {
                                inventory.remove(&item);
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
    input(&load_string("inputs/2023/day2.input"))
        .into_iter()
        .map(|(_, sets)| {
            let mut nums: HashMap<_, _> = [(Red, 0), (Green, 0), (Blue, 0)].into();
            for set in sets {
                for (item, amount) in set {
                    if amount > nums[&item] {
                        nums.insert(item, amount);
                    }
                }
            }
            nums.values().copied().product::<usize>()
        })
        .sum()
}
