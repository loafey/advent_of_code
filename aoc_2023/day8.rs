use utils::{load_string, NumExt};
use std::collections::HashMap;

fn input(s: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let (insts, moves) = s.split_once("\n\n").unwrap();
    let moves = moves
        .lines()
        .map(|a| {
            let [i, a, b] = a
                .split([' ', '=', ',', ')', '('])
                .filter(|s| !s.is_empty())
                .array_chunks::<3>()
                .next()
                .unwrap();
            (i, (a, b))
        })
        .collect::<HashMap<_, _>>();
    (insts, moves)
}

fn calc(insts: &str, moves: &HashMap<&str, (&str, &str)>, start: &str, end: &str) -> usize {
    let mut pos = start;
    insts
        .chars()
        .cycle()
        .enumerate()
        .find(|(_, inst)| {
            pos = match inst {
                'L' => moves[pos].0,
                'R' => moves[pos].1,
                _ => unreachable!(),
            };
            pos.ends_with(end)
        })
        .map(|(u, _)| u + 1)
        .unwrap_or_default()
}

pub fn part1() -> usize {
    let binding = load_string("inputs/2023/day8.input");
    let (insts, moves) = input(&binding);
    calc(insts, &moves, "AAA", "ZZZ")
}
pub fn part2() -> usize {
    let binding = load_string("inputs/2023/day8.input");
    let (insts, moves) = input(&binding);
    moves
        .keys()
        .filter(|a| a.ends_with('A'))
        .map(|p| (calc(insts, &moves, p, "Z")))
        .reduce(|a, b| a.lcm(&b))
        .unwrap_or_default()
}
