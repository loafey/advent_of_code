use arrayvec::ArrayVec;
use std::cmp::Ordering::*;
use utils::bi_functors::BiFunctorExtExt;

fn input() -> ([[bool; 100]; 100], &'static str) {
    let (rules_input, input) = include_str!("../inputs/2024/day5.input")
        .split_once("\n\n")
        .unwrap();

    let mut rules: [[bool; 100]; 100] = unsafe { std::mem::zeroed() };
    for r in rules_input.lines() {
        let (a, b) = r
            .split_once('|')
            .unwrap()
            .splet(|s| s.parse::<usize>().unwrap());
        rules[a][b] = true;
    }
    (rules, input)
}

pub fn part1() -> usize {
    let (rules, input) = input();
    let mut sum = 0;
    let mut visited: [bool; 100];
    'outer: for inp in input.lines().filter(|s| !s.is_empty()) {
        visited = unsafe { std::mem::zeroed() };
        let nums = inp
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<ArrayVec<_, 23>>();

        for num in &nums {
            for (i, r) in rules[*num].iter().enumerate() {
                if *r && visited[i] {
                    continue 'outer;
                }
            }
            visited[*num] = true;
        }
        sum += nums[nums.len() / 2]
    }

    sum
}
pub fn part2() -> usize {
    let (rules, input) = input();
    let mut sum = 0;
    'outer: for inp in input.lines().filter(|s| !s.is_empty()) {
        let mut visited: [bool; 100] = unsafe { std::mem::zeroed() };
        let nums = inp
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<ArrayVec<_, 23>>();

        for num in &nums {
            for (i, r) in rules[*num].iter().enumerate() {
                if *r && visited[i] {
                    let mut nums = nums;
                    nums.sort_by(|a, b| match rules[*a][*b] {
                        true => Less,
                        false => Greater,
                    });
                    sum += nums[nums.len() / 2];

                    continue 'outer;
                }
            }

            visited[*num] = true;
        }
    }

    sum
}
