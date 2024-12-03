fn solve(enable_do: bool) -> i64 {
    let input = include_str!("../inputs/2024/day3.input");
    let mut muls = Vec::new();
    let mut nums = Vec::new();
    let mut curr = String::new();
    let mut num_curr = String::new();
    let mut enabled = true;

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
                enabled = curr == "do(";
                curr = String::new();
            }
            (',', "mul(") => {
                curr.push(c);
                nums.push(num_curr);
                num_curr = String::new()
            }
            (')', "mul(,") => {
                nums.push(num_curr);
                num_curr = String::new();
                curr = String::new();
                if enabled || !enable_do {
                    muls.push((
                        nums.pop().unwrap().parse().unwrap(),
                        nums.pop().unwrap().parse().unwrap(),
                    ));
                }
            }
            (c, "mul(") if c.is_numeric() => num_curr.push(c),
            (c, "mul(,") if c.is_numeric() => num_curr.push(c),
            _ => {
                curr = String::new();
                nums = Vec::new();
                num_curr = String::new();
            }
        }
    }
    muls.into_iter().map(|(a, b): (i64, i64)| a * b).sum()
}

pub fn part1() -> i64 {
    solve(false)
}
pub fn part2() -> i64 {
    solve(true)
}
