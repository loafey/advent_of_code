use rayon::prelude::*;
static mut COLORS: Vec<&'static str> = Vec::new();

fn setup() -> impl Iterator<Item = &'static str> {
    let (colors, designs) = include_str!("../inputs/2024/day19.input")
        .split_once("\n\n")
        .unwrap();

    let colors = colors.split(", ").collect::<Vec<_>>();
    unsafe { COLORS = colors };
    designs.lines().filter(|s| !s.is_empty())
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
    setup()
        .par_bridge()
        .map(|design| (rec(design) > 0) as usize)
        .sum()
}
pub fn part2() -> usize {
    setup().par_bridge().map(rec).sum()
}
