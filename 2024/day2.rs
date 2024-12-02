use std::cmp::{Ordering, Ordering::*};

pub fn part1() -> i64 {
    let mut safe = 0;
    'outer: for row in include_str!("../inputs/2024/day2.input")
        .lines()
        .map(|r| r.split_whitespace().map(|s| s.parse::<i64>().unwrap()))
    {
        let mut last: Option<(i64, Ordering)> = None;
        for num in row.into_iter() {
            match last {
                Some((lst, dir)) => {
                    let diff = lst - num;
                    if diff.abs() > 3
                        || lst == num
                        || matches!(
                            (dir, lst.cmp(&num)),
                            (Equal, Equal) | (Less, Greater) | (Greater, Less)
                        )
                    {
                        continue 'outer;
                    }
                    last = Some((num, lst.cmp(&num)))
                }
                None => last = Some((num, Ordering::Equal)),
            }
        }
        safe += 1;
    }
    safe
}
pub fn part2() -> i64 {
    let mut safe = 0;
    'outerouter: for row in include_str!("../inputs/2024/day2.input")
        .lines()
        .map(|r| r.split_whitespace().map(|s| s.parse::<i64>().unwrap()))
    {
        let row = row.collect::<Vec<_>>();
        'outer: for row_i in 0..row.len() {
            let mut row = row.clone();
            row.remove(row_i);

            let mut last: Option<(i64, Ordering)> = None;
            for num in row {
                if let Some((lst, dir)) = last {
                    let diff = lst - num;
                    if diff.abs() > 3
                        || lst == num
                        || matches!(
                            (dir, lst.cmp(&num)),
                            (Equal, Equal) | (Less, Greater) | (Greater, Less)
                        )
                    {
                        continue 'outer;
                    }
                    last = Some((num, lst.cmp(&num)))
                } else {
                    last = Some((num, Ordering::Equal));
                }
            }
            safe += 1;
            continue 'outerouter;
        }
    }
    safe
}
