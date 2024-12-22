use rayon::prelude::*;
use std::collections::VecDeque;

trait Nummy {
    fn mix(self, rhs: Self) -> Self;
    fn prune(self) -> Self;
}
impl Nummy for i64 {
    fn mix(self, rhs: Self) -> Self {
        rhs ^ self
    }

    fn prune(self) -> Self {
        self % 16777216
    }
}

pub fn part1() -> i64 {
    let codes = include_str!("../inputs/2024/day22.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap());
    let mut sum = 0;
    for mut code in codes {
        for _ in 0..2000 {
            code = math(code).0;
        }
        sum += code;
    }
    sum
}

fn math(code: i64) -> (i64, i64, i64) {
    let p1 = code.mix(code * 64).prune();
    let p2 = p1.mix(p1 / 32).prune();
    let p3 = p2.mix(p2 * 2048).prune();
    let price = p3 % 10;
    let change = (p3 % 10) - (code % 10);
    (p3, price, change)
}

pub fn part2() -> i64 {
    let codes = include_str!("../inputs/2024/day22.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    (-9..=9)
        .into_par_iter()
        .map(|x| {
            let mut max_sum = 0;
            for y in -9..=9 {
                if x == y {
                    continue;
                }
                for z in -9..=9 {
                    if y == z {
                        continue;
                    }
                    for w in -9..=9 {
                        if w == z {
                            continue;
                        }
                        let sequence = [x, y, z, w];
                        let mut sum = 0;
                        for code in &codes {
                            let mut code = *code;
                            let mut seq = VecDeque::new();
                            for _ in 0..2000 {
                                let (nc, price, change) = math(code);
                                seq.push_back(change);
                                if seq.len() > 4 {
                                    seq.pop_front();
                                }
                                if seq == sequence {
                                    sum += price;
                                    break;
                                }
                                code = nc;
                            }
                        }
                        max_sum = max_sum.max(sum);
                    }
                }
            }
            max_sum
        })
        .max()
        .unwrap_or_default()
}
