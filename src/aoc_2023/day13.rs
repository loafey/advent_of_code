use std::{
    cmp::Ordering,
    io::{stdout, Write},
};

use rayon::iter::{
    IntoParallelIterator, IntoParallelRefIterator, ParallelBridge, ParallelIterator as _,
};

use crate::utils::load_string;

fn check(row: &[char], nums: &[usize]) -> bool {
    let s = row
        .split(|c| matches!(c, '.'))
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    if nums.len() != s.len() {
        return false;
    }
    for (s, size) in s.into_iter().zip(nums) {
        if s.len() != *size {
            return false;
        }
    }
    true
}

fn check_amount(row: &[char], nums: &[usize]) -> usize {
    row.split(|c| matches!(c, '.' | '?'))
        .filter(|s| !s.is_empty())
        .count()
        .min(nums.len())
}

fn perm(mut row: Vec<char>, nums: &[usize]) -> usize {
    fn perm(row: &mut [char], indicies: &[usize], nums: &[usize]) -> usize {
        print!("checkie: ");
        row[..].iter().for_each(|p| print!("{p}"));
        println!();
        match indicies {
            [x, xs @ ..] => {
                let amount = check_amount(&row[..*x], nums);
                let check = check(&row[..*x], &nums[..amount]);
                if !check {
                    println!("{:?} {:?} {}", &row[..*x], &nums[..amount], amount);
                    return 0;
                }
                // print!(
                //     "{}\t",
                //     (0..=nums.len()).any(|i| check(&row[..*x], &nums[..i]))
                // );
                // row[..*x].iter().for_each(|p| print!("{p}"));
                // print!(" ");
                // print!(
                //     "{:?}\t",
                //     (0..=nums.len()).map(|i| &nums[..i]).collect::<Vec<_>>()
                // );
                // println!();
                let mut res = 0;
                row[*x] = '.';
                res += perm(row, xs, nums);
                row[*x] = '#';
                res += perm(row, xs, nums);
                res
            }
            [] => {
                if check(row, nums) {
                    1
                } else {
                    0
                }
            }
        }
    }
    let mut indices = row
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == '?')
        .map(|(u, _)| u)
        .collect::<Vec<_>>();

    perm(&mut row, &indices, nums)
}

pub fn part1() -> usize {
    let binding = load_string("inputs/2023/day12.input");
    let inp = binding
        .lines()
        .map(|r| {
            let (row, nums) = r.split_once(' ').unwrap();
            let row = row.to_string();
            let nums = nums
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let row = row.chars().collect::<Vec<_>>();
            (nums, row)
        })
        .collect::<Vec<_>>();

    inp.into_iter().map(|(nums, row)| perm(row, &nums)).sum()
}

pub fn part2() -> usize {
    0
    // let binding = load_string("inputs/2023/day12.input");
    // let inp = binding
    //     .lines()
    //     .map(|r| {
    //         let (row, nums) = r.split_once(' ').unwrap();
    //         let row = row.to_string();
    //         let mut nums = nums
    //             .split(',')
    //             .map(|s| s.parse::<usize>().unwrap())
    //             .collect::<Vec<_>>();
    //         let mut row = row.chars().collect::<Vec<_>>();
    //         let num_clone = nums.clone();
    //         let row_clone = row.clone();
    //         for _ in 0..4 {
    //             row.push('?');
    //             row.append(&mut row_clone.clone());
    //             nums.append(&mut num_clone.clone());
    //         }
    //         (nums, row)
    //     })
    //     .collect::<Vec<_>>();
    // inp.into_par_iter()
    //     .map(|(nums, row)| perm(row, &nums))
    //     .sum()
}
