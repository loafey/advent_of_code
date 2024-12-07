use arrayvec::ArrayVec;
use rayon::prelude::*;
use utils::Concat;

fn oppify(result: i64, vals: &[i64], cc: bool, acc: i64) -> Option<i64> {
    if acc > result {
        return None;
    }
    match vals {
        [] => match acc == result {
            true => Some(acc),
            false => None,
        },
        [x, rest @ ..] => match cc {
            true => oppify(result, rest, cc, acc + x)
                .or_else(|| oppify(result, rest, cc, acc * x))
                .or_else(|| oppify(result, rest, cc, acc.concat(*x))),
            false => {
                oppify(result, rest, cc, acc + x).or_else(|| oppify(result, rest, cc, acc * x))
            }
        },
    }
}

fn calc(cc: bool) -> i64 {
    let data = include_str!("../inputs/2024/day7.input");

    data.lines()
        .par_bridge()
        .filter_map(|l| {
            let (result, vals) = l.split_once(':').unwrap();
            let result = result.parse::<i64>().unwrap();
            let vals = vals
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<ArrayVec<_, 12>>();

            oppify(result, &vals, cc, 0)
        })
        .sum()
}

pub fn part1() -> i64 {
    calc(false)
}
pub fn part2() -> i64 {
    calc(true)
}
