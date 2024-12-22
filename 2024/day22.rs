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
        // print!("{code}: ");
        for _ in 0..2000 {
            let p1 = code.mix(code * 64).prune();
            let p2 = p1.mix(p1 / 32).prune();
            let p3 = p2.mix(p2 * 2048).prune();
            code = p3;
        }
        sum += code;
        // println!("{code}");
    }
    sum
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
                for z in -9..=9 {
                    for w in -9..=9 {
                        let sequence = [x, y, z, w];
                        let mut sum = 0;
                        for code in &codes {
                            let mut code = *code;
                            // print!("{code}: ");
                            // println!("{}", code % 10);
                            let mut seq = VecDeque::new();
                            for _ in 0..2000 {
                                let p1 = code.mix(code * 64).prune();
                                let p2 = p1.mix(p1 / 32).prune();
                                let p3 = p2.mix(p2 * 2048).prune();
                                let price = p3 % 10;
                                let change = (p3 % 10) - (code % 10);
                                seq.push_back(change);
                                if seq.len() > 4 {
                                    seq.pop_front();
                                }
                                // println!("{} ({})", p3 % 10, (p3 % 10) - (code % 10));
                                if seq == sequence {
                                    sum += price;
                                    break;
                                }
                                code = p3;
                            }
                            // println!("{code}");
                        }
                        max_sum = max_sum.max(sum);
                    }
                }
            }
            println!("x = {x} done");
            max_sum
        })
        .max()
        .unwrap_or_default()
}
// > 1608
