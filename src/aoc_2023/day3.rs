use crate::utils::load_string;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Map {
    Num(usize),
    Gear,
    Symbol,
    Empty,
}
impl From<&str> for Map {
    fn from(s: &str) -> Self {
        if s == "." {
            Empty
        } else if s == "*" {
            Gear
        } else if let Ok(o) = s.parse() {
            Num(o)
        } else {
            Symbol
        }
    }
}
use Map::*;

fn inputs(s: String) -> Vec<Vec<Map>> {
    s.lines()
        .map(|s| {
            let mut stack: Vec<Map> = Vec::new();
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
        .collect::<Vec<_>>()
}

fn get_neighbors(x: usize, y: usize, inputs: &Vec<Vec<Map>>) -> Vec<usize> {
    // top
    let mut neighbors = Vec::new();
    if y > 0 {
        neighbors.push(&inputs[y - 1][x]);
    }

    // bottom
    if y < inputs.len() {
        neighbors.push(&inputs[y + 1][x]);
    }

    // left
    if x > 0 {
        neighbors.push(&inputs[y][x - 1]);
    }

    // right
    if x < inputs[y].len() {
        neighbors.push(&inputs[y][x + 1]);
    }

    // Top left
    if x > 0 && y > 0 {
        neighbors.push(&inputs[y - 1][x - 1]);
    }

    // Top right
    if x < inputs[y].len() && y > 0 {
        neighbors.push(&inputs[y - 1][x + 1]);
    }

    // Bottom left
    if x > 0 && y < inputs.len() {
        neighbors.push(&inputs[y + 1][x - 1]);
    }

    // Bottom right
    if x < inputs[y].len() && y < inputs.len() {
        neighbors.push(&inputs[y + 1][x + 1]);
    }
    let mut neighbors = neighbors
        .into_iter()
        .filter_map(|s| match s {
            Num(u) => Some(*u),
            _ => None,
        })
        .collect::<Vec<_>>();
    neighbors.sort();
    neighbors.dedup();
    neighbors
}

pub fn part1() -> usize {
    let inputs = inputs(load_string("inputs/2023/day3.input"));

    let mut sum = 0;
    for y in 0..inputs.len() {
        for x in 0..inputs[y].len() {
            if !matches!(inputs[y][x], Num(_)) && inputs[y][x] != Empty {
                let neighbors = get_neighbors(x, y, &inputs);
                for n in neighbors {
                    sum += n;
                }
            }
        }
    }

    sum
}
pub fn part2() -> usize {
    let inputs = inputs(load_string("inputs/2023/day3.input"));

    let mut sum = 0;
    for y in 0..inputs.len() {
        for x in 0..inputs[y].len() {
            if !matches!(inputs[y][x], Num(_)) && inputs[y][x] != Empty {
                let neighbors = get_neighbors(x, y, &inputs);
                if inputs[y][x] == Gear && neighbors.len() == 2 {
                    sum += neighbors[0] * neighbors[1];
                }
            }
        }
    }

    sum
}
