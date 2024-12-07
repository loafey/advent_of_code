use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Op {
    Mul,
    Add,
    Concat,
}

fn oppify(len: usize, cc: bool) -> Vec<Vec<Op>> {
    match len {
        0 => {
            if cc {
                vec![vec![Op::Mul], vec![Op::Add], vec![Op::Concat]]
            } else {
                vec![vec![Op::Mul], vec![Op::Add]]
            }
        }
        x => {
            if cc {
                let mut next = oppify(x - 1, cc);
                let mut res = next.clone();
                let mut tres = next.clone();

                next.iter_mut().for_each(|s| s.push(Op::Add));
                res.iter_mut().for_each(|s| s.push(Op::Mul));
                tres.iter_mut().for_each(|s| s.push(Op::Concat));

                next.append(&mut res);
                next.append(&mut tres);
                next
            } else {
                let mut next = oppify(x - 1, cc);
                let mut res = next.clone();

                next.iter_mut().for_each(|s| s.push(Op::Add));
                res.iter_mut().for_each(|s| s.push(Op::Mul));

                next.append(&mut res);
                next
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
            let vals = vals
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            let oppify = oppify(vals.len() - 2, cc);
            let mut res = 0;
            for op in oppify {
                let mut i = 0;
                let ans = vals
                    .iter()
                    .copied()
                    .reduce(|acc, v| {
                        i += 1;
                        match op[i - 1] {
                            Op::Mul => acc * v,
                            Op::Add => acc + v,
                            Op::Concat => {
                                let mut pow = 10;
                                while v >= pow {
                                    pow *= 10;
                                }
                                acc * pow + v
                            }
                        }
                    })
                    .unwrap();
                if ans == result {
                    res = ans;
                    break;
                }
            }
            res
        })
        .sum()
}

pub fn part1() -> i64 {
    calc(false)
}
pub fn part2() -> i64 {
    calc(true)
}
