use crate::utils::load_string;

pub fn part1() -> i64 {
    load_string("inputs/2023/day1.input")
        .lines()
        .map(|s| {
            let mut s = s.chars().filter(|c| c.is_ascii_digit());
            let first = s.next().unwrap();
            let next = if let Some(c) = s.last() { c } else { first };
            format!("{first}{next}").parse::<i64>().unwrap()
        })
        .sum()
}

pub fn part2() -> i64 {
    load_string("inputs/2023/day1.input")
        .replace("two", "two2two")
        .replace("one", "one1one")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
        .lines()
        .map(|s| {
            let mut s = s.chars().filter(|c| c.is_ascii_digit());
            let first = s.next().unwrap();
            let next = if let Some(c) = s.last() { c } else { first };
            format!("{first}{next}").parse::<i64>().unwrap()
        })
        .sum()
}
