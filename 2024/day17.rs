use Combo::*;
use OpCodes::*;
#[derive(Debug)]
enum Combo {
    Lit(i64),
    A,
    B,
    C,
    Unreachable,
}

#[derive(Debug)]
enum OpCodes {
    Adv(Combo), // 0
    Bxl(i64),   // 1
    Bst(Combo), // 2
    Jnz(Combo), // 3
    Bxc,        // 4
    Out(Combo), // 5
    Bdv(Combo), // 6
    Cdv(Combo), // 7
}
impl From<(i64, i64)> for OpCodes {
    fn from((op, value): (i64, i64)) -> Self {
        let combo = match value {
            0..=3 => Lit(value),
            4 => A,
            5 => B,
            6 => C,
            7 => Unreachable,
            _ => panic!(),
        };
        match op {
            0 => Adv(combo),
            1 => Bxl(value),
            2 => Bst(combo),
            3 => Jnz(combo),
            4 => Bxc,
            5 => Out(combo),
            6 => Bdv(combo),
            7 => Cdv(combo),
            _ => panic!(),
        }
    }
}

fn solve(mut a: i64, ins: &[i64]) -> String {
    let mut b = 0;
    let mut c = 0;
    let mut ip = 0;
    macro_rules! val {
        ($cv:expr) => {
            match $cv {
                Combo::Lit(v) => v,
                Combo::A => a,
                Combo::B => b,
                Combo::C => c,
                Combo::Unreachable => panic!(),
            }
        };
    }

    let mut output = Vec::new();
    while ip < ins.len() {
        let op = OpCodes::from((ins[ip], ins[ip + 1]));
        match op {
            Adv(cv) => a /= 2i64.pow(val!(cv) as u32),
            Bxl(cv) => b ^= cv,
            Bst(cv) => b = val!(cv) % 8,
            Jnz(cv) => {
                if a != 0 {
                    let j = val!(cv);
                    ip = j as usize;
                    continue;
                }
            }
            Bxc => b ^= c,
            Out(cv) => output.push(format!("{}", val!(cv) % 8)),
            Bdv(cv) => b = a / 2i64.pow(val!(cv) as u32),
            Cdv(cv) => c = a / 2i64.pow(val!(cv) as u32),
        }

        ip += 2;
    }
    output.join(",")
}

pub fn part1() -> String {
    let (regs, ins) = include_str!("../inputs/2024/day17.input")
        .split_once("\n\n")
        .unwrap();
    let mut regs = regs
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok());

    let ins = ins
        .split([' ', ','])
        .skip(1)
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    solve(regs.next().unwrap(), &ins)
}
pub fn part2() -> i64 {
    let ins = include_str!("../inputs/2024/day17.input")
        .split_once("\n\n")
        .unwrap()
        .1;

    let ins = ins
        .split([' ', ','])
        .skip(1)
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let ins_string = ins
        .iter()
        .map(|s| format!("{s}"))
        .collect::<Vec<_>>()
        .join(",");

    fn math(a: i64) -> i64 {
        let mut b = a % 8;
        b ^= 4;
        let c = a / (1 << b);
        b ^= c;
        b ^= 4;
        b % 8
    }
    let input = ins.iter().copied().rev().collect::<Vec<_>>();
    fn rec(base: i64, input: &[i64]) -> Vec<i64> {
        if input.is_empty() {
            return vec![base];
        }

        let mut ans = Vec::new();
        for a in (base * 8)..=(base * 8 + 7) {
            if math(a) == input[0] {
                // println!("{a}: {}", math(a));
                let mut r = rec(a, &input[1..]);
                ans.append(&mut r);
            }
        }
        ans
    }
    let mut ans = rec(0, &input);
    ans.sort();

    for a in ans {
        let output = solve(a, &ins);
        if output == ins_string {
            return a;
        }
    }
    0
}
