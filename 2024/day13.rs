// use rayon::prelude::*;
use utils::NumExt;

#[derive(Debug)]
struct Game {
    a: (i64, i64),
    b: (i64, i64),
    goal: (i64, i64),
}

fn solve((ax, ay): (i64, i64), (bx, by): (i64, i64), (gx, gy): (i64, i64)) -> Option<(i64, i64)> {
    // println!("{a:?} {b:?} {g:?}");
    let mut b_div = gx / bx;
    println!("starting a at: {b_div}");
    let mut binary = 1;
    let binary_mod = 2;
    loop {
        // println!("{b_div} {binary}");
        // std::thread::sleep(std::time::Duration::from_millis(200));
        let x_diff = gx / (bx * b_div);
        let y_diff = gy / (by * b_div);
        if x_diff >= 1 && y_diff >= 1 {
            if binary != 1 {
                b_div += binary / binary_mod;
                b_div -= 1;
                binary = 1;
            } else {
                break;
            }
        } else {
            b_div -= binary;
            binary *= binary_mod;
        }

        if b_div <= 0 {
            if binary != 1 {
                b_div += binary / binary_mod;
                binary = 1;
            } else {
                return None;
            }
        }
    }

    let mut binary = 1;
    let mut a_div = gx / ax;
    println!("starting b at: {b_div}");
    loop {
        let x_diff = gx as f64 / ((bx * b_div) + (ax * a_div)) as f64;
        let y_diff = gy as f64 / ((by * b_div) + (ay * a_div)) as f64;

        println!("{binary}: {a_div} {b_div} {x_diff} {y_diff}");
        if x_diff != 1.0 || y_diff != 1.0 {
            println!("bin!");
            a_div -= binary;
            binary *= binary_mod;
            if a_div == 0 {
                b_div -= 1;
                a_div = gx / ax;
                binary = 1;
            }
        }
        if x_diff == 1.0 && y_diff == 1.0 {
            break;
        }

        if b_div <= 0 {
            if binary != 1 {
                a_div += binary / binary_mod;
                binary = 1;
            } else {
                return None;
            }
        }
    }
    Some((b_div, a_div))
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
