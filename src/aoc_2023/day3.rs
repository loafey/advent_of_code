use crate::utils::{load_string, matrix_get};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Map {
    Num(usize),
    Gear,
    Symbol,
    Empty,
}
impl From<&str> for Map {
    fn from(s: &str) -> Self {
        match s {
            "." => Empty,
            "*" => Gear,
            s => match s.parse() {
                Ok(o) => Num(o),
                _ => Symbol,
            },
        }
    }
}
use Map::*;

fn inputs(s: String) -> Vec<Vec<Map>> {
    s.lines()
        .map(|s| {
            let mut stack = Vec::new();
            let mut word = String::new();
            for c in s.chars() {
                if c == '.' {
                    if !word.is_empty() {
                        stack.push((&word[..]).into());
                    }
                    stack.push(".".into());
                    word = String::new();
                } else if c.is_numeric() {
                    word.push(c);
                } else {
                    if !word.is_empty() {
                        stack.push((&word[..]).into());
                    }
                    stack.push((&c.to_string()[..]).into());
                    word = String::new();
                }
            }
            if !word.is_empty() {
                stack.push((&word[..]).into());
            }
            let mut i = 0;
            while i < stack.len() {
                if let Num(n) = stack[i] {
                    let len = format!("{n}").len();
                    for l in 0..(len - 1) {
                        stack.insert(i + l, stack[i]);
                    }
                    i += len;
                } else {
                    i += 1;
                }
            }
            stack
        })
        .collect()
}

fn get_neighbors(x: usize, y: usize, inputs: &[Vec<Map>]) -> Vec<usize> {
    let mut neighbors = ((-1..=1).flat_map(|y| (-1..=1).map(move |x| (y, x))))
        .filter(|c| !matches!(c, (0, 0)))
        .filter_map(|(ymod, xmod)| matrix_get(y, x, ymod, xmod, inputs))
        .filter_map(|m| match m {
            Num(n) => Some(n),
            _ => None,
        })
        .collect::<Vec<_>>();
    neighbors.sort();
    neighbors.dedup();
    neighbors
}

fn work(inputs: &[Vec<Map>], f: fn(Map, Vec<usize>) -> usize) -> usize {
    (0..inputs.len())
        .flat_map(|y| {
            (0..inputs[y].len())
                .filter(move |x| !matches!(&inputs[y][*x], Num(_) | Empty))
                .map(move |x| f(inputs[y][x], get_neighbors(x, y, inputs)))
        })
        .sum()
}

pub fn part1() -> usize {
    work(&inputs(load_string("inputs/2023/day3.input")), |_, v| {
        v.into_iter().sum()
    })
}
pub fn part2() -> usize {
    work(&inputs(load_string("inputs/2023/day3.input")), |m, v| {
        (m == Gear && v.len() == 2)
            .then(|| v[0] * v[1])
            .unwrap_or_default()
    })
}
