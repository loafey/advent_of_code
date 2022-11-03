use std::io::Write;

fn power_level(input: i32, x: i32, y: i32) -> i32 {
    let mut power_level = x + 10;
    power_level *= y;
    power_level += input;
    power_level *= x + 10;
    power_level = (power_level.rem_euclid(1000)) / 100;
    power_level -= 5;

    power_level
}

const AREA_SIZE: i32 = 300;
fn calc_power_box(input: i32, size: i32, x: i32, y: i32) -> i32 {
    (y..y + size)
        .flat_map(|y| (x..x + size).map(move |x| power_level(input, x, y)))
        .sum()
}
pub fn part1() -> String {
    let input = include_str!("input/day11.input").parse::<i32>().unwrap();

    let (mut max_x, mut max_y, mut max_pow) = (0, 0, 0);
    for (x, y) in (1..=AREA_SIZE).flat_map(|x| (1..=AREA_SIZE).map(move |y| (x, y))) {
        let pow = calc_power_box(input, 3, x, y);
        if pow > max_pow {
            max_x = x;
            max_y = y;
            max_pow = pow;
        }
    }

    format!("{max_x},{max_y}")
}

pub fn part2() -> String {
    let input = include_str!("input/day11.input").parse::<i32>().unwrap();

    let (mut max_x, mut max_y, mut max_pow, mut max_size) = (0, 0, 0, 0);
    for size in 10..=30 {
        for (x, y) in
            (1..=AREA_SIZE - size).flat_map(|x| (1..=AREA_SIZE - size).map(move |y| (x, y)))
        {
            let pow = calc_power_box(input, size, x, y);
            if pow > max_pow {
                max_x = x;
                max_y = y;
                max_pow = pow;
                max_size = size;
            }
        }
    }

    format!("{max_x},{max_y},{max_size}")
}
