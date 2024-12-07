#[derive(Debug, Clone, Copy)]
enum Op {
    Mul,
    Add,
    Concat,
}

fn oppify(len: usize) -> Vec<Vec<Op>> {
    match len {
        0 => vec![vec![Op::Mul], vec![Op::Add]],
        x => {
            let mut next = oppify(x - 1);
            let mut res = next.clone();

            next.iter_mut().for_each(|s| s.insert(0, Op::Add));
            res.iter_mut().for_each(|s| s.insert(0, Op::Mul));
            next.append(&mut res);
            next
        }
    }
}

fn oppify_cc(len: usize) -> Vec<Vec<Op>> {
    match len {
        0 => vec![vec![Op::Mul], vec![Op::Add], vec![Op::Concat]],
        x => {
            let mut next = oppify_cc(x - 1);
            let mut res = next.clone();
            let mut tres = next.clone();

            next.iter_mut().for_each(|s| s.insert(0, Op::Add));
            res.iter_mut().for_each(|s| s.insert(0, Op::Mul));
            tres.iter_mut().for_each(|s| s.insert(0, Op::Concat));
            next.append(&mut res);
            next.append(&mut tres);
            next
        }
    }
}

pub fn part1() -> i64 {
    let data = include_str!("../inputs/2024/day7.input");

    let mut sum = 0;
    for l in data.lines() {
        let (result, vals) = l.split_once(':').unwrap();
        let result = result.parse::<i64>().unwrap();
        let vals = vals
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        for op in oppify(vals.len() - 2) {
            let mut i = 0;
            let ans = vals
                .iter()
                .copied()
                .reduce(|acc, v| {
                    i += 1;
                    match op[i - 1] {
                        Op::Mul => acc * v,
                        Op::Add => acc + v,
                        _ => panic!(),
                    }
                })
                .unwrap();
            if ans == result {
                sum += ans;
                break;
            }
        }
    }

    sum
}
pub fn part2() -> i64 {
    let data = include_str!("../inputs/2024/day7.input");

    let mut sum = 0;
    for l in data.lines() {
        let (result, vals) = l.split_once(':').unwrap();
        let result = result.parse::<i64>().unwrap();
        let vals = vals
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let oppify_cc = oppify_cc(vals.len() - 2);
        for op in oppify_cc {
            let mut i = 0;
            let ans = vals
                .iter()
                .copied()
                .reduce(|acc, v| {
                    i += 1;
                    match op[i - 1] {
                        Op::Mul => acc * v,
                        Op::Add => acc + v,
                        Op::Concat => format!("{acc}{v}").parse::<i64>().unwrap(),
                    }
                })
                .unwrap();
            if ans == result {
                sum += ans;
                break;
            }
        }
    }

    sum
}
