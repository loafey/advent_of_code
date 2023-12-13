use crate::utils::{load_string, MatrixTools};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ReflectionLine {
    None,
    VerticalSingle(usize),
    HorizontalSingle(usize),
    Vertical(usize),
    Horizontal(usize),
}
impl ReflectionLine {
    fn value(self) -> usize {
        match self {
            None => 0,
            VerticalSingle(v) => v,
            Vertical(v) => v,
            HorizontalSingle(h) => h * 100,
            Horizontal(h) => h * 100,
        }
    }
}
use ReflectionLine::*;

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

fn diff<T: PartialEq + Eq>(a: &[T], b: &[T]) -> usize {
    if a.len() != b.len() {
        return usize::MAX;
    }

    a.iter()
        .zip(b.iter())
        .map(|(a, b)| if a != b { 1 } else { 0 })
        .sum()
}

fn check(
    block: &[Vec<char>],
    f: fn(usize) -> ReflectionLine,
    fsingle: fn(usize) -> ReflectionLine,
    single: bool,
) -> ReflectionLine {
    let mut rl = None;
    let mut res = Vec::new();
    let mut pos = 0;
    'crugno: for (i, x) in block.windows(2).enumerate() {
        pos += 1;
        if let [r1, r2] = x {
            if diff(r1, r2) <= 1 {
                let (mut last_x, mut last_y) = (0, 0);
                let mut mut_dif = 0;
                for (x, y) in (0..i + 1).rev().zip(i + 1..block.len()) {
                    last_x = x;
                    last_y = y;
                    mut_dif += diff(&block[x], &block[y]);
                    if mut_dif > 1 {
                        continue 'crugno;
                    }
                }
                if mut_dif == 0 {
                    res.push(f(pos));
                } else if mut_dif == 1 && single {
                    res.push(fsingle(pos));
                }
            }
        }
    }

    res.into_iter()
        .find(|rl| {
            (!single && matches!(rl, Horizontal(..) | Vertical(..)))
                || (single && matches!(rl, HorizontalSingle(..) | VerticalSingle(..)))
        })
        .unwrap_or(None)
}

fn check_block(block: &[Vec<char>], single: bool) -> ReflectionLine {
    let mut rl = check(block, Horizontal, HorizontalSingle, single);
    if rl == None {
        let block = block.to_vec().rotate();
        rl = check(&block, Vertical, VerticalSingle, single);
    }
    rl
}

pub fn part1() -> usize {
    parse()
        .into_iter()
        .map(|block| check_block(&block, false))
        .filter(|r| matches!(r, Horizontal(..) | Vertical(..)))
        .map(|r| r.value())
        .sum()
}
pub fn part2() -> usize {
    parse()
        .into_iter()
        .map(|block| check_block(&block, true))
        .filter(|r| matches!(r, HorizontalSingle(..) | VerticalSingle(..)))
        .map(|r| r.value())
        .sum()
}
