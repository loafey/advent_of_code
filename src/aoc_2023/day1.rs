use crate::{parser::parse, utils::load_string};

pub fn part1() -> i64 {
    let inp = load_string("src/aoc_2023/day1.input");
    ((parse(&inp) | "\n\n") ^ parse ^ (|s| (!(s | '\n')) - (0, |a, b| a + b))) - (0, i64::max)
}

pub fn part2() -> i32 {
    let inp = load_string("src/aoc_2023/day1.input");
    (parse(&inp) / "\n\n")
        << (
            |s| (s | "") - (0, |a, _| a + 1),
            |s| (s | "") - (0, |a, _| a + 1),
        )
        >> (|a, b| a + b)
}
