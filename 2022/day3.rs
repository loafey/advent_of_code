use std::hint::unreachable_unchecked;
use utils::load_string;

pub fn part1() -> i32 {
    load_string("inputs/2022/day3.input")
        .lines()
        .map(|s| {
            let len = s.len();
            let side_1 = &s[0..len / 2].chars().collect::<Vec<_>>();
            let dupe = s[len / 2..len]
                .chars()
                .find(|c| side_1.contains(c))
                .unwrap();
            char_to_value(dupe)
        })
        .sum()
}

fn char_to_value(c: char) -> i32 {
    (c as u8 - if c.is_uppercase() { 38 } else { 96 }) as i32
}

pub fn part2() -> i32 {
    load_string("inputs/2022/day3.input")
        .lines()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|s| {
            let [s1, s2, s3] = s else {
                unsafe { unreachable_unchecked() }
            };
            let b2 = s2.chars().collect::<Vec<_>>();
            let b3 = s3.chars().collect::<Vec<_>>();
            s1.chars()
                .find(|c| b2.contains(c) && b3.contains(c))
                .unwrap()
        })
        .map(char_to_value)
        .sum()
}
