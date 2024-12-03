pub fn part1() -> i64 {
    let input = include_str!("../inputs/2024/day3.input");
    let mut muls = Vec::new();

    let mut nums = Vec::new();
    let mut curr = String::new();
    let mut num_curr = String::new();
    for c in input.chars() {
        // println!("-------------------: {c}\n{muls:?}\n{nums:?}\n{curr:?}\n{num_curr:?}");
        match c {
            'm' if curr.is_empty() => curr.push(c),
            'u' if curr == "m" => curr.push(c),
            'l' if curr == "mu" => curr.push(c),
            '(' if curr == "mul" => curr.push(c),
            ',' if curr == "mul(" => {
                curr.push(c);
                nums.push(num_curr);
                num_curr = String::new()
            }
            ')' if curr == "mul(," => {
                nums.push(num_curr);
                num_curr = String::new();
                curr = String::new();
                muls.push((
                    nums.pop().unwrap().parse().unwrap(),
                    nums.pop().unwrap().parse().unwrap(),
                ));
            }
            c if c.is_numeric() && (curr == "mul(" || curr == "mul(,") => num_curr.push(c),
            _ => {
                curr = String::new();
                nums = Vec::new();
                num_curr = String::new();
            }
        }
    }

    muls.into_iter().map(|(a, b): (i64, i64)| a * b).sum()
}
pub fn part2() -> i64 {
    let input = include_str!("../inputs/2024/day3.input");
    let mut muls = Vec::new();

    let mut nums = Vec::new();
    let mut curr = String::new();
    let mut num_curr = String::new();
    let mut enabled = true;
    for c in input.chars() {
        // println!("-------------------: {c}\n{muls:?}\n{nums:?}\n{curr:?}\n{num_curr:?}");
        match c {
            'd' if curr.is_empty() => curr.push(c),
            'o' if curr == "d" => curr.push(c),
            'n' if curr == "do" => curr.push(c),
            '\'' if curr == "don" => curr.push(c),
            't' if curr == "don'" => curr.push(c),
            'm' if curr.is_empty() => curr.push(c),
            'u' if curr == "m" => curr.push(c),
            'l' if curr == "mu" => curr.push(c),
            '(' if curr == "mul" => curr.push(c),
            '(' if curr == "do" || curr == "don't" => curr.push(c),
            ')' if curr == "do(" => {
                curr = String::new();
                enabled = true;
            }
            ')' if curr == "don't(" => {
                curr = String::new();
                enabled = false;
            }
            ',' if curr == "mul(" => {
                curr.push(c);
                nums.push(num_curr);
                num_curr = String::new()
            }
            ')' if curr == "mul(," => {
                nums.push(num_curr);
                num_curr = String::new();
                curr = String::new();
                if enabled {
                    muls.push((
                        nums.pop().unwrap().parse().unwrap(),
                        nums.pop().unwrap().parse().unwrap(),
                    ));
                }
            }
            c if c.is_numeric() && (curr == "mul(" || curr == "mul(,") => num_curr.push(c),
            _ => {
                curr = String::new();
                nums = Vec::new();
                num_curr = String::new();
            }
        }
    }

    muls.into_iter().map(|(a, b): (i64, i64)| a * b).sum()
}
