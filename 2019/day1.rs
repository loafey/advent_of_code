use utils::load_string;

pub fn part1() -> i32 {
    load_string("inputs/2019/day1.input")
        .lines()
        .map(|s| s.parse::<f32>().unwrap())
        .map(calc)
        .sum()
}

fn calc(m: f32) -> i32 {
    (m / 3.0).floor() as i32 - 2
}

fn calc2(m: f32) -> i32 {
    let mut total = calc(m);
    let mut m = total;
    while calc(m as f32) > 0 {
        total += calc(m as f32);
        m = calc(m as f32);
    }
    total
}

pub fn part2() -> i32 {
    load_string("inputs/2019/day1.input")
        .lines()
        .map(|s| s.parse::<f32>().unwrap())
        .map(calc2)
        .sum()
}
