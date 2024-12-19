#![allow(static_mut_refs)]
// why memoize this when you can have mutable global variables :)
static mut COLORS: Vec<&'static str> = Vec::new();

use rayon::prelude::*;

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
        .map(f)
        .sum()
}

#[memoize::memoize]
fn rec(d: &'static str) -> usize {
    if d.is_empty() {
        return 1;
    }
    unsafe { &COLORS }
        .iter()
        .map(|c| d.strip_prefix(c).map(rec).unwrap_or_default())
        .sum()
}

pub fn part1() -> usize {
    solve(|d| (rec(d) > 0) as usize)
}
pub fn part2() -> usize {
    solve(rec)
}
