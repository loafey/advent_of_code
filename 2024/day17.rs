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
    let mut ip = 0;

    let mut a = regs.next().unwrap();
    let mut b = regs.next().unwrap();
    let mut c = regs.next().unwrap();

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
pub fn part2() -> i64 {
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
    let ins_string = ins
        .iter()
        .map(|s| format!("{s:02x}"))
        .collect::<Vec<_>>()
        .join(",");
    let mut ip = 0;

    let _a_orig = regs.next().unwrap();
    // let mut start =
    //              0b0000000000000000010_110000011111111111111111111111111111111000000; // extremly close
    //              0b0000000000000000010_110110011111111111111111111111111111111000000; //25986278 * 1353950;
    let mut start = 8i64.pow(15);
    let mut a = 0;
    let mut b = regs.next().unwrap();
    let mut c = regs.next().unwrap();

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

    // for i in 0.. {
    //     let mut m = 8i64.pow(15) - i;
    //     let mut c = 0;
    //     while m != 0 {
    //         m /= 8;
    //         c += 1
    //     }
    //     if c < 16 {
    //         println!("{c}: {}", 8i64.pow(14) - i);
    //         break;
    //     } else {
    //         println!("{c}: {}", 8i64.pow(14) - i)
    //     }
    // }

    // 0

    let mut test = 0;
    let test_amount = 100000;
    loop {
        if test == test_amount {
            println!("\ntesting: {a} ({a:064b})");
        }
        ip = 0;
        b = 0;
        c = 0;
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
                Out(cv) => output.push(format!("{:02x}", val!(cv) % 8)),
                Bdv(cv) => b = a / 2i64.pow(val!(cv) as u32),
                Cdv(cv) => c = a / 2i64.pow(val!(cv) as u32),
            }

            ip += 2;
        }

        let output = output.join(",");
        if output == ins_string {
            return start - 1;
        } else {
            if test == test_amount {
                println!("got:      {output}");
                println!("expected: {ins_string}");
            }
            test += 1;
            if test > test_amount {
                test = 0;
            }
            start += 1;
            a = start;
        }
    }
}
