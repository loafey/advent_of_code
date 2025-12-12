#[allow(clippy::type_complexity)]
fn input() -> (Vec<usize>, Vec<((usize, usize), Vec<usize>)>) {
    let mut shapes = Vec::new();
    let mut goals = Vec::new();
    for block in include_str!("../inputs/2025/day12.input").split("\n\n") {
        if block.contains('#') {
            shapes.push(
                block
                    .lines()
                    .skip(1)
                    .map(|l| l.chars().filter(|c| *c == '#').count())
                    .sum(),
            )
        } else {
            for l in block.lines() {
                let (size, indexes) = l.split_once(':').unwrap();
                let indexes = indexes
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                let (x, y) = size.split_once('x').unwrap();
                let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
                goals.push(((x, y), indexes));
            }
        }
    }
    (shapes, goals)
}

pub fn part1() -> u64 {
    let (shapes, goals) = input();
    let goals: Vec<(_, usize)> = goals
        .into_iter()
        .map(|(matrix, goals)| {
            (
                matrix,
                goals
                    .into_iter()
                    .enumerate()
                    .map(|(i, a)| shapes[i] * a)
                    .sum::<usize>(),
            )
        })
        .collect::<Vec<_>>();
    goals
        .into_iter()
        .map(|((x, y), indexes)| (indexes <= x * y) as u64)
        .sum()
}

pub fn part2() -> u64 {
    2025
}
