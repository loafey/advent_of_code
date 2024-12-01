use utils::{load_string, MatrixGet as _};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Spot {
    Empty,
    Rock,
    Boulder,
}

impl std::fmt::Debug for Spot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Rock => write!(f, "#"),
            Self::Boulder => write!(f, "O"),
        }
    }
}
use Spot::*;
fn tilt(table: &mut [Vec<Spot>], y_dir: isize, x_dir: isize) -> bool {
    let mut moved = false;
    for y in 0..table.len() {
        for x in 0..table[y].len() {
            if table[y][x] == Boulder {
                if let Some(Empty) = table.matrix_get(y, x, y_dir, x_dir) {
                    table[(y as isize + y_dir) as usize][(x as isize + x_dir) as usize] = Boulder;
                    table[y][x] = Empty;
                    moved = true;
                }
            }
        }
    }
    moved
}

pub fn part1() -> usize {
    let mut inp = load_string("inputs/2023/day14.input")
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Empty,
                    '#' => Rock,
                    'O' => Boulder,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // print_rocks(&inp);
    while tilt(&mut inp, -1, 0) {}
    // println!();
    // print_rocks(&inp);
    value(&inp)
}

fn value(table: &[Vec<Spot>]) -> usize {
    let len = table.len();
    table
        .iter()
        .enumerate()
        .map(|(y, r)| r.iter().filter(|s| matches!(s, Boulder)).count() * (len - y))
        .sum()
}

pub fn part2() -> usize {
    let mut inp = load_string("inputs/2023/day14.input")
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Empty,
                    '#' => Rock,
                    'O' => Boulder,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut set = HashSet::new();
    let mut clean_set = HashMap::new();
    let mut try_1 = true;
    let mut simulate = false;
    let mut i = 0;
    while i < VAL {
        while tilt(&mut inp, -1, 0) {}
        while tilt(&mut inp, 0, -1) {}
        while tilt(&mut inp, 1, 0) {}
        while tilt(&mut inp, 0, 1) {}

        if !simulate {
            if let Some(i2) = clean_set.get(&inp)
                && !try_1
            {
                i = VAL - (VAL - i - 1) % (i - i2);
                simulate = true;
                continue;
            } else if set.contains(&inp) && try_1 && clean_set.contains_key(&inp) {
                clean_set = HashMap::new();
                try_1 = false;
            } else if !set.contains(&inp) {
                set.insert(inp.clone());
            } else {
                clean_set.insert(inp.clone(), i);
            }
        }
        i += 1;
    }
    value(&inp)
}

const VAL: usize = 1_000_000_000;
