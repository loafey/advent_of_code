use utils::load_string;

fn solver(s: &str) -> i64 {
    let mut s = s.chars().filter(|c| c.is_ascii_digit());
    let first = s.next().unwrap();
    let next = if let Some(c) = s.next_back() {
        c
    } else {
        first
    };
    format!("{first}{next}").parse::<i64>().unwrap()
}

pub fn part1() -> i64 {
    load_string("inputs/2023/day1.input")
        .lines()
        .map(solver)
        .sum()
}

pub fn part2() -> i64 {
    load_string("inputs/2023/day1.input")
        .replace("two", "t2o")
        .replace("one", "o1e")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
        .lines()
        .map(solver)
        .sum()
}
