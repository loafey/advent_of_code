use std::collections::HashSet;

use crate::utils::parse_next;
#[derive(Debug)]
struct Beacon {
    x: isize,
    y: isize,
}
#[derive(Debug)]
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
fn create_line(a: (isize, isize), b: (isize, isize)) -> Box<dyn Fn(isize) -> isize> {
    let d = (b.1 - a.1) as f64 / (b.0 - a.0) as f64;
    let m = b.1 - (d * b.0 as f64).round() as isize;
    Box::new(move |x: isize| (d * x as f64).round() as isize + m)
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
    let min_x = bs.iter().map(|(s, b)| s.x.min(b.x) - 20).min().unwrap();
    let max_x = bs.iter().map(|(s, b)| s.x.max(b.x) + 20).max().unwrap();
    let mut row = vec![Spot::Empty; (max_x - min_x) as usize];
    let b_len = bs.len() - 1;
    bs.into_iter().enumerate().for_each(|(i, (s, b))| {
        if s.y == Y {
            row[(s.x - min_x) as usize] = Spot::Sensor;
        }
        if b.y == Y {
            row[(b.x - min_x) as usize] = Spot::Beacon;
        }
        println!("{i} {b_len}");
        let mut i = 0;

        loop {
            i += 1;
            let bottom = (s.x, s.y + i);
            let right = (s.x + i, s.y);
            let top = (s.x, s.y - i);
            let left = (s.x - i, s.y);
            let mut buf: HashSet<_> = [bottom, right, top, left].into();

            for x in left.0..top.0 {
                let y = create_line(left, top)(x);
                buf.insert((x, y));
            }
            for x in left.0..bottom.0 {
                let y = create_line(left, bottom)(x);
                buf.insert((x, y));
            }
            for x in top.0..right.0 {
                let y = create_line(top, right)(x);
                buf.insert((x, y));
            }
            for x in bottom.0..right.0 {
                let y = create_line(bottom, right)(x);
                buf.insert((x, y));
            }
            let mut found_beacon = false;

            buf.into_iter().for_each(|(x, y)| {
                if y == Y && row[(x - min_x) as usize] == Spot::Empty {
                    row[(x - min_x) as usize] = Spot::Occupied;
                }
                if y == b.y && x == b.x {
                    found_beacon = true;
                }
            });
            if found_beacon {
                break;
            }
        }
    });

    // row.iter().for_each(|s| print!("{s:?}"));
    // println!();
    row.into_iter().filter(|s| *s == Spot::Occupied).count()
}

pub fn part2() -> i32 {
    0
}
