use rayon::prelude::*;
use utils::first;

#[derive(Debug)]
struct Game {
    a: (i64, i64),
    b: (i64, i64),
    goal: (i64, i64),
}

#[memoize::memoize]
fn solve(
    cur: (i64, i64),
    a: (i64, i64),
    b: (i64, i64),
    g: (i64, i64),
    p: (usize, usize),
) -> Option<(i64, i64, usize, usize)> {
    let (curx, cury) = cur;
    let (ax, ay) = a;
    let (bx, by) = b;
    let (gx, gy) = g;
    let (pa, pb) = p;
    if ax + curx == gx && ay + cury == gy {
        return Some((gx, gy, pa + 1, pb));
    } else if bx + curx == gx && by + cury == gy {
        return Some((gx, gy, pa, pb + 1));
    }
    if curx > gx || cury > gy {
        return None;
    }

    first![
        solve((curx + ax, cury + ay), a, b, g, (pa + 1, pb)),
        solve((curx + bx, cury + by), a, b, g, (pa, pb + 1))
    ]
}

fn parse_and_solve(xmod: i64, ymod: i64) -> usize {
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
        .into_par_iter()
        .filter_map(
            |Game {
                 a,
                 b,
                 goal: (gx, gy),
             }| solve((0, 0), a, b, (gx + xmod, gy + ymod), (0, 0)),
        )
        .map(|(_, _, a, b)| a * 3 + b)
        .sum()
}

pub fn part1() -> usize {
    parse_and_solve(0, 0)
}
pub fn part2() -> usize {
    parse_and_solve(10000000000000, 10000000000000)
}
