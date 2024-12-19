#[memoize::memoize]
fn rec(design: &'static str, designs: Vec<&'static str>) -> usize {
    if design.is_empty() {
        return 1;
    }

    let mut okay = 0;
    for d in &designs {
        if let Some(suffix) = design.strip_prefix(d) {
            okay += rec(suffix, designs.clone());
        }
    }

    okay
}

pub fn part1() -> usize {
    let (colors, designs) = include_str!("../inputs/2024/day19.input")
        .split_once("\n\n")
        .unwrap();

    let colors = colors.split(", ").collect::<Vec<_>>();
    let designs = designs.lines().filter(|s| !s.is_empty());

    let mut sum = 0;
    for design in designs {
        sum += (rec(design, colors.clone()) > 0) as usize;
    }
    sum
}
pub fn part2() -> usize {
    let (colors, designs) = include_str!("../inputs/2024/day19.input")
        .split_once("\n\n")
        .unwrap();

    let colors = colors.split(", ").collect::<Vec<_>>();
    let designs = designs.lines().filter(|s| !s.is_empty());

    let mut sum = 0;
    for design in designs {
        sum += rec(design, colors.clone()) as usize;
    }
    sum
}
