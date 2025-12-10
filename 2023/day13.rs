use utils::{MatrixTrans, SliceTools, load_string};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ReflectionLine {
    None,
    Vertical(usize),
    Horizontal(usize),
}
use ReflectionLine::*;
impl ReflectionLine {
    fn value(self) -> usize {
        match self {
            None => 0,
            Vertical(v) => v,
            Horizontal(h) => h * 100,
        }
    }
}

fn parse() -> Vec<Vec<Vec<char>>> {
    load_string("inputs/2023/day13.input")
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|b| {
            b.lines()
                .map(|s| s.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn check(block: &[Vec<char>], f: fn(usize) -> ReflectionLine, single: bool) -> ReflectionLine {
    'crugno: for (i, x) in block.windows(2).enumerate() {
        if let [r1, r2] = x
            && r1.diff(r2).unwrap() <= 1
        {
            let mut mut_dif = 0;
            for (x, y) in (0..i + 1).rev().zip(i + 1..block.len()) {
                mut_dif += block[x].diff(&block[y]).unwrap();
                if mut_dif > 1 {
                    continue 'crugno;
                }
            }
            if mut_dif == 0 && !single || mut_dif == 1 && single {
                return f(i + 1);
            }
        }
    }

    None
}

fn check_block(block: &[Vec<char>], single: bool) -> ReflectionLine {
    let mut rl = check(block, Horizontal, single);
    if rl == None {
        let block = block.to_vec().rotate();
        rl = check(&block, Vertical, single);
    }
    rl
}

fn solve(single: bool) -> usize {
    parse()
        .into_iter()
        .map(|block| check_block(&block, single))
        .map(|r| r.value())
        .sum()
}

pub fn part1() -> usize {
    solve(false)
}
pub fn part2() -> usize {
    solve(true)
}
