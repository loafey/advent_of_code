// use rayon::prelude::*;
use utils::NumExt;

#[derive(Debug)]
struct Game {
    a: (i64, i64),
    b: (i64, i64),
    goal: (i64, i64),
}

fn solve((ax, ay): (i64, i64), (bx, by): (i64, i64), (gx, gy): (i64, i64)) -> Option<(i64, i64)> {
    let m = (ax * gy - gx * ay) / (ax * by - bx * ay);
    let n = (gx - m * bx) / ax;
    if n * ax + m * bx == gx && n * ay + m * by == gy {
        Some((m, n))
    } else {
        None
    }
}

fn parse_and_solve(xmod: i64, ymod: i64) -> i64 {
    let input = include_str!("../inputs/2024/day13.input")
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|r| {
            let mut l = r.lines();
            let a = l.next().unwrap();
            let mut a = a
                .split([' ', 'X', '+', ',', 'Y', ':', 'A', 'B'])
                .filter(|s| !s.is_empty())
                .skip(1);
            let a1 = a.next().unwrap().parse::<i64>().unwrap();
            let a2 = a.next().unwrap().parse::<i64>().unwrap();

            let b = l.next().unwrap();
            let mut b = b
                .split([' ', 'X', '+', ',', 'Y', ':', 'A', 'B'])
                .filter(|s| !s.is_empty())
                .skip(1);
            let b1 = b.next().unwrap().parse::<i64>().unwrap();
            let b2 = b.next().unwrap().parse::<i64>().unwrap();

            let p = l.next().unwrap();
            let mut p = p
                .split([' ', 'X', '+', ',', 'Y', ':', 'A', 'B', '='])
                .filter(|s| !s.is_empty())
                .skip(1);
            let p1 = p.next().unwrap().parse::<i64>().unwrap();
            let p2 = p.next().unwrap().parse::<i64>().unwrap();

            Game {
                a: (a1, a2),
                b: (b1, b2),
                goal: (p1, p2),
            }
        })
        .collect::<Vec<_>>();

    input
        .into_iter()
        .filter_map(
            |Game {
                 a,
                 b,
                 goal: (gx, gy),
             }| solve(a, b, (gx + xmod, gy + ymod)),
        )
        .map(|(b, a)| a * 3 + b)
        .sum()
}

pub fn part1() -> i64 {
    parse_and_solve(0, 0)
}
pub fn part2() -> i64 {
    parse_and_solve(10000000000000, 10000000000000)
}
