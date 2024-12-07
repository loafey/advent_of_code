use arrayvec::ArrayVec;
use rayon::prelude::*;
use utils::{first, Concat};

fn oppify(vals: &[i64], cc: bool, result: i64, acc: i64) -> Option<i64> {
    if acc > result {
        return None;
    }
    match vals {
        [] => match acc == result {
            true => Some(acc),
            false => None,
        },
        [x, rest @ ..] => {
            let oppify = |acc| oppify(rest, cc, result, acc);
            match cc {
                true => first!(oppify(acc + x), oppify(acc * x), oppify(acc.concat(*x))),
                false => first!(oppify(acc + x), oppify(acc * x)),
            }
        }
    }
}

fn calc(cc: bool) -> i64 {
    let data = include_str!("../inputs/2024/day7.input");
    data.lines()
        .par_bridge()
        .map(|l| {
            let (result, vals) = l.split_once(':').unwrap();
            let result = result.parse::<i64>().unwrap();
            (
                result,
                vals.split_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect::<ArrayVec<_, 12>>(),
            )
        })
        .filter_map(|(result, vals)| oppify(&vals, cc, result, 0))
        .sum()
}

pub fn part1() -> i64 {
    calc(false)
}
pub fn part2() -> i64 {
    calc(true)
}
