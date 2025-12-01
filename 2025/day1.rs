#[derive(Debug)]
enum Turn {
    Left(i64),
    Right(i64),
}

fn input() -> Vec<Turn> {
    include_str!("../inputs/2025/day1.input")
        .lines()
        .map(|l| {
            let (dir, num) = l.split_at(1);
            let num = num.parse::<i64>().unwrap();
            match dir {
                "L" => Turn::Left(num),
                "R" => Turn::Right(num),
                _ => unreachable!(),
            }
        })
        .collect()
}

pub fn part1() -> i64 {
    let mut value = 50;
    let mut pass = 0;
    for turn in input() {
        value = match turn {
            Turn::Left(v) => value - v,
            Turn::Right(v) => value + v,
        }
        .rem_euclid(100);
        pass += (value == 0) as i64;
    }
    pass
}

pub fn part2() -> i64 {
    let mut value: i64 = 50;
    let mut pass = 0;
    for turn in input() {
        let (moddy, cap) = match turn {
            Turn::Left(v) => (-1, v),
            Turn::Right(v) => (1, v),
        };
        // this is slow, but i am too tired to fix it atm :)
        for _ in 0..cap {
            value += moddy;
            value = value.rem_euclid(100);
            pass += (value == 0) as i64;
        }
    }
    pass
}
