#![allow(static_mut_refs, mutable_transmutes)]
// why memoize this when you can have mutable global variables :)
static mut COLORS: Vec<&'static str> = Vec::new();
#[thread_local]
static CACHE: LazyCell<Cache> = LazyCell::new(Cache::default);

use rayon::prelude::*;
use rustc_hash::FxHashMap;
type Cache = FxHashMap<usize, usize>;
use std::{cell::LazyCell, mem::transmute};

fn solve(f: fn(&'static str) -> usize) -> usize {
    let (colors, designs) = include_str!("../inputs/2024/day19.input")
        .split_once("\n\n")
        .unwrap();

    let colors = colors.split(", ").collect::<Vec<_>>();
    unsafe { COLORS = colors };
    designs
        .lines()
        .filter(|s| !s.is_empty())
        .par_bridge()
        .map(|d| {
            unsafe { transmute::<&Cache, &mut Cache>(&*CACHE) }.clear();
            f(d)
        })
        .sum()
}

fn rec(d: &'static str) -> usize {
    if let Some(v) = CACHE.get(&d.len()) {
        return *v;
    } else if d.is_empty() {
        return 1;
    }
    let u = unsafe { &COLORS }
        .iter()
        .map(|c| d.strip_prefix(c).map(rec).unwrap_or_default())
        .sum::<usize>();
    unsafe { transmute::<&Cache, &mut Cache>(&*CACHE) }.insert(d.len(), u);
    u
}

pub fn part1() -> usize {
    solve(|d| (rec(d) > 0) as usize)
}
pub fn part2() -> usize {
    solve(rec)
}
