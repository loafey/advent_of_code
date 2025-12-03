use std::{
    cell::{OnceCell, RefCell},
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

use memoize::memoize;

fn input() -> impl Iterator<Item = Vec<u64>> {
    include_str!("../inputs/2025/day3.input")
        .lines()
        .map(|c| c.chars().map(|c| c as u64 - 0x30).collect::<Vec<_>>())
}

pub fn part1() -> u64 {
    let mut sum = 0;
    for bank in input() {
        let mut max = 0;
        for i in 0..bank.len() {
            for j in i + 1..bank.len() {
                max = max.max((bank[i] * 10) + bank[j]);
            }
        }
        sum += max;
    }
    sum
}

#[thread_local]
static mut CACHE: OnceCell<RefCell<HashMap<u64, u64>>> = OnceCell::new();

fn solve(count: u8, data: Vec<u64>) -> u64 {
    unsafe {
        let mut hasher = DefaultHasher::new();
        count.hash(&mut hasher);
        data.hash(&mut hasher);
        let hash = hasher.finish();
        if let Some(res) = CACHE.get_or_init(Default::default).borrow().get(&hash) {
            // println!("cache");
            return *res;
        }
    }
    if count == 1 {
        return data.into_iter().max().unwrap();
    }
    let mut max = 0;
    for i in 0..data.len() {
        let mut data = data.clone();

        let v = data[i];
        data = data[i + 1..].to_vec();
        if data.len() + 1 < count as usize {
            continue;
        }
        let ans = v * 10u64.pow(count as u32 - 1) + solve(count - 1, data);
        max = max.max(ans)
    }
    let mut hasher = DefaultHasher::new();
    count.hash(&mut hasher);
    data.hash(&mut hasher);
    let hash = hasher.finish();
    unsafe {
        CACHE.get_mut().unwrap().borrow_mut().insert(hash, max);
    }
    max
}

pub fn part2() -> u64 {
    let mut sum = 0;
    for (i, bank) in input().enumerate() {
        unsafe {
            if let Some(c) = CACHE.get_mut() {
                // c.borrow_mut().clear();
            }
        };
        let ans = solve(12, bank);
        println!("bank: {i}: {}", ans);
        sum += ans;
    }
    println!("3121910778619");
    sum
}
