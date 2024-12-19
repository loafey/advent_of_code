use rayon::prelude::*;
static mut COLORS: Vec<&'static str> = Vec::new();

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
fn rec(design: &'static str) -> usize {
    if design.is_empty() {
        return 1;
    }

    let mut okay = 0;
    #[allow(static_mut_refs)]
    for d in unsafe { &COLORS } {
        if let Some(suffix) = design.strip_prefix(d) {
            okay += rec(suffix);
        }
    }

    okay
}

pub fn part1() -> usize {
    solve(|design| (rec(design) > 0) as usize)
}
pub fn part2() -> usize {
    solve(rec)
}
