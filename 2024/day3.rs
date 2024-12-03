use arrayvec::{ArrayString, ArrayVec};

fn solve(enable_do: bool) -> i64 {
    let input = include_str!("../inputs/2024/day3.input");
    let mut nums: ArrayVec<_, 6> = ArrayVec::new();
    let mut curr: ArrayString<6> = ArrayString::new();
    let mut num_curr: ArrayString<3> = ArrayString::new();
    let mut enabled = true;

    let mut sum = 0;
    for c in input.chars() {
        match (c, &*curr) {
            ('d', "")
            | ('o', "d")
            | ('n', "do")
            | ('\'', "don")
            | ('t', "don'")
            | ('m', "")
            | ('u', "m")
            | ('l', "mu")
            | ('(', "mul")
            | ('(', "do")
            | ('(', "don't") => curr.push(c),
            (')', "do(") | (')', "don't(") => {
                enabled = *curr == *"do(";
                curr.clear();
            }
            (',', "mul(") => {
                curr.push(c);
                nums.push(num_curr.parse::<i64>().unwrap());
                num_curr.clear();
            }
            (')', "mul(,") if !num_curr.is_empty() => {
                nums.push(num_curr.parse::<i64>().unwrap());
                num_curr.clear();
                curr.clear();
                if enabled || !enable_do {
                    sum += nums.pop().unwrap() * nums.pop().unwrap();
                }
            }
            (c, "mul(") if c.is_numeric() => num_curr.push(c),
            (c, "mul(,") if c.is_numeric() => num_curr.push(c),
            _ => {
                curr.clear();
                nums.clear();
                num_curr.clear();
            }
        }
    }
    sum
}

pub fn part1() -> i64 {
    solve(false)
}
pub fn part2() -> i64 {
    solve(true)
}
