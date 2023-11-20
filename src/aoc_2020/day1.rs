use crate::utils::load_string;

pub fn part1() -> i64 {
    let s = load_string("inputs/2020/day1.input")
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    for x in &s {
        for y in &s {
            if *x + *y == 2020 {
                return *x * *y;
            }
        }
    }
    0
}
pub fn part2() -> i64 {
    let s = load_string("inputs/2020/day1.input")
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    for x in &s {
        for y in &s {
            for z in &s {
                if *x + *y + *z == 2020 {
                    return *x * *y * *z;
                }
            }
        }
    }
    0
}
