use std::cmp::{Ordering, Ordering::*};
use utils::{BooleanToCoolean, CoolBool, FindSome, True};

fn is_alright(nums: impl Iterator<Item = i64>) -> CoolBool {
    let mut last: Option<(i64, Ordering)> = None;
    for num in nums {
        match last {
            Some((lst, dir)) => {
                ((lst - num).abs() > 3
                    || lst == num
                    || matches!(
                        (dir, lst.cmp(&num)),
                        (Equal, Equal) | (Less, Greater) | (Greater, Less)
                    ))
                .c()?;
                last = Some((num, lst.cmp(&num)))
            }
            None => last = Some((num, Equal)),
        }
    }
    True
}

pub fn part1() -> usize {
    include_str!("../inputs/2024/day2.input")
        .lines()
        .map(|r| r.split_whitespace().map(|s| s.parse::<i64>().unwrap()))
        .filter_map(is_alright)
        .count()
}
pub fn part2() -> usize {
    include_str!("../inputs/2024/day2.input")
        .lines()
        .map(|r| r.split_whitespace().map(|s| s.parse::<i64>().unwrap()))
        .filter_map(|row| {
            let row = row.collect::<Vec<_>>();
            (0..row.len()).find_some(|row_i| {
                let mut row = row.clone();
                row.remove(row_i);
                is_alright(row.into_iter())
            })
        })
        .count()
}
