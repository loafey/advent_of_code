use std::collections::HashSet;

use num_bigint::BigInt;

use crate::utils::{manhattan_distance, parse_next};
#[derive(Debug, Clone, Copy)]
struct Beacon {
    x: isize,
    y: isize,
}
#[derive(Debug, Clone, Copy)]
struct Sensor {
    x: isize,
    y: isize,
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Spot {
    Sensor,
    Beacon,
    Empty,
    Occupied,
}
impl std::fmt::Debug for Spot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sensor => write!(f, "S"),
            Self::Beacon => write!(f, "B"),
            Self::Empty => write!(f, "."),
            Self::Occupied => write!(f, "#"),
        }
    }
}
fn create_line(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
    let d = (b.1 - a.1) as f64 / (b.0 - a.0) as f64;
    let m = b.1 - (d * b.0 as f64).round() as isize;
    (d as isize, m)
}

pub fn part1() -> usize {
    const Y: isize = 2000000;
    let bs = include_str!("input/day15.input")
        .lines()
        .map(|r| {
            let mut splat = r
                .split(|c: char| c.is_alphabetic() || c == ',' || c == '=' || c == ':' || c == ' ')
                .filter(|s| !s.is_empty());
            (
                Sensor {
                    x: parse_next(&mut splat),
                    y: parse_next(&mut splat),
                },
                Beacon {
                    x: parse_next(&mut splat),
                    y: parse_next(&mut splat),
                },
            )
        })
        .collect::<Vec<_>>();
    let min_x = bs
        .iter()
        .map(|(s, b)| s.x.min(b.x) - 2000000)
        .min()
        .unwrap();
    let max_x = bs
        .iter()
        .map(|(s, b)| s.x.max(b.x) + 2000000)
        .max()
        .unwrap();
    let mut row = vec![Spot::Empty; (max_x - min_x) as usize];

    for (s, b) in bs.iter() {
        if b.y == Y {
            row[(b.x - min_x) as usize] = Spot::Beacon
        }
        if s.y == Y {
            row[(s.x - min_x) as usize] = Spot::Sensor
        }
    }

    let b_len = bs.len() - 1;
    bs.into_iter()
        .enumerate()
        .map(|(i, (s, b))| (i, (s, b), manhattan_distance((s.x, s.y), (b.x, b.y))))
        .filter(|(_, (s, _), size)| (Y <= s.y + size) && (Y >= s.y - size))
        .map(|(i, (s, _), size)| (i, s, size))
        .for_each(|(i, Sensor { x: s_x, y: s_y }, size)| {
            println!("{i}/{b_len}: ({s_x}, {s_y}), {size}");
            let is_top = Y < s_y;
            let (d, m) = create_line(
                if is_top {
                    (s_x, s_y - size)
                } else {
                    (s_x - size, s_y)
                },
                if is_top {
                    (s_x + size, s_y)
                } else {
                    (s_x, s_y + size)
                },
            );
            let x = (Y - m) / d;
            let dif = (s_x - x).abs();
            //println!("{dif} ");
            let range = s_x - dif..=s_x + dif;
            //println!("{range:?}");
            for x in range {
                if row[(x - min_x) as usize] == Spot::Empty {
                    row[(x - min_x) as usize] = Spot::Occupied
                }
            }
            //println!()
        });

    //row.iter().for_each(|s| print!("{s:?}"));
    //println!();
    //println!("...................###S#############.###########.........");
    row.into_iter().filter(|s| *s == Spot::Occupied).count()
}

pub fn part2() -> BigInt {
    let bs = include_str!("input/day15.input")
        .lines()
        .map(|r| {
            let mut splat = r
                .split(|c: char| c.is_alphabetic() || c == ',' || c == '=' || c == ':' || c == ' ')
                .filter(|s| !s.is_empty());
            (
                Sensor {
                    x: parse_next(&mut splat),
                    y: parse_next(&mut splat),
                },
                Beacon {
                    x: parse_next(&mut splat),
                    y: parse_next(&mut splat),
                },
            )
        })
        .collect::<Vec<_>>();

    let max = 20;
    let mut last = String::new();
    for x in 0..max {
        let p = format!("{:.3}", ((x as f64 / max as f64) * 100.0));
        if p != last {
            last = p;
            println!("{last}%");
        }
        for y in 0..max {
            let mut contains = true;
            for (Sensor { x: s_x, y: s_y }, size) in bs
                .iter()
                .map(|(s, b)| ((*s, *b), manhattan_distance((s.x, s.y), (b.x, b.y))))
                .filter(|((s, _), size)| (y <= s.y + size) && (y >= s.y - size))
                .map(|((s, _), size)| (s, size))
            {
                contains = contains && manhattan_distance((x, y), (s_x, s_y)) > size;
                if !contains {
                    break;
                }
            }
            if contains {
                return BigInt::from(x) * BigInt::from(4000000) + BigInt::from(y);
            }
            //println!();
        }
    }

    BigInt::from(0)
}
