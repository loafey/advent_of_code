use crate::utils::load_string;

pub fn part1() -> usize {
    load_string("src/aoc_2020/day2.input")
        .lines()
        .map(|s| {
            s.split(|c| matches!(c, ' ' | '-' | ':'))
                .array_chunks::<5>()
                .filter(|[s, e, c, _, str]| {
                    let s = s.parse::<usize>().unwrap();
                    let e = e.parse::<usize>().unwrap();
                    let c = str.chars().filter(move |o| *c == o.to_string()).count();
                    s <= c && c <= e
                })
                .count()
        })
        .sum()
}
pub fn part2() -> usize {
    load_string("src/aoc_2020/day2.input")
        .lines()
        .map(|s| {
            s.split(|c| matches!(c, ' ' | '-' | ':'))
                .array_chunks::<5>()
                .filter(|[s, e, c, _, str]| {
                    let c = c.chars().next().unwrap();
                    let s = s.parse::<usize>().unwrap() - 1;
                    let e = e.parse::<usize>().unwrap() - 1;
                    let chars = str.chars().collect::<Vec<_>>();
                    (chars[s] == c && chars[e] != c) || (chars[s] != c && chars[e] == c)
                })
                .count()
        })
        .sum()
}
