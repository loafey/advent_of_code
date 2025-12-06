use utils::MatrixTrans;

#[derive(Debug)]
enum Op {
    Mul,
    Add,
}

fn p1_parser() -> Vec<(Vec<u64>, Op)> {
    let mut output: Vec<(Vec<u64>, Op)> = Vec::new();
    for l in include_str!("../inputs/2025/day6.input").lines() {
        if l.contains('*') || l.contains('+') {
            for (i, v) in l.split_whitespace().enumerate() {
                let v = match v {
                    "*" => Op::Mul,
                    "+" => Op::Add,
                    _ => unreachable!(),
                };
                if let Some(vec) = output.get_mut(i) {
                    vec.1 = v;
                } else {
                    output.push((Vec::new(), v));
                }
            }
        } else {
            for (i, v) in l.split_whitespace().enumerate() {
                if let Some(vec) = output.get_mut(i) {
                    vec.0.push(v.parse().unwrap());
                } else {
                    output.push((vec![v.parse().unwrap()], Op::Mul));
                }
            }
        }
    }
    output
}

fn p2_parser() -> Vec<(Vec<u64>, Op)> {
    let mut output: Vec<(Vec<u64>, Op)> = Vec::new();
    let matrix: Vec<Vec<char>> = include_str!("../inputs/2025/day6.input")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let matrix = matrix.transpose();
    for group in matrix.split(|c| c.iter().all(|c| c.is_whitespace())) {
        let mut op = Op::Add;
        let mut nums = Vec::new();
        for (i, l) in group.iter().enumerate() {
            if i == 0 {
                op = match l[l.len() - 1] {
                    '+' => Op::Add,
                    '*' => Op::Mul,
                    _ => unreachable!(),
                };
            }
            let num: u64 = l[0..l.len() - 1]
                .iter()
                .collect::<String>()
                .trim()
                .parse()
                .unwrap();
            nums.push(num);
        }
        output.push((nums, op));
    }
    output
}

fn solve(input: fn() -> Vec<(Vec<u64>, Op)>) -> u64 {
    let mut sum = 0;
    for (v, op) in input() {
        let mut iter = v.into_iter();
        let mut result = iter.next().unwrap();
        for v in iter {
            match op {
                Op::Mul => result *= v,
                Op::Add => result += v,
            }
        }
        sum += result
    }
    sum
}

pub fn part1() -> u64 {
    solve(p1_parser)
}

pub fn part2() -> u64 {
    solve(p2_parser)
}
