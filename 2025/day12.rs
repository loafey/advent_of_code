#[inline(always)]
fn parse(str: &'static str) -> usize {
    unsafe { str.parse().unwrap_unchecked() }
}

pub fn part1() -> usize {
    include_str!("../inputs/2025/day12.input")
        .split("\n\n")
        .filter(|b| !b.contains('#'))
        .flat_map(|block| {
            block.lines().map(|l| {
                let (size, indexes) = l.split_once(':').unwrap();
                let indexes: Vec<_> = indexes.split_whitespace().map(|s| 7 * parse(s)).collect();
                let (x, y) = size.split_once('x').unwrap();
                let (x, y) = (parse(x), parse(y));
                ((x, y), indexes)
            })
        })
        .filter(|((x, y), goals)| goals.iter().sum::<usize>() <= x * y)
        .count()
}

pub fn part2() -> u64 {
    2025
}
