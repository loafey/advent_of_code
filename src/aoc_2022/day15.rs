use std::collections::{BTreeSet, HashSet};

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
        .map(|(s, b)| s.x.min(b.x) - manhattan_distance((s.x, s.y), (b.x, b.y)))
        .min()
        .unwrap();
    let max_x = bs
        .iter()
        .map(|(s, b)| s.x.max(b.x) + manhattan_distance((s.x, s.y), (b.x, b.y)))
        .max()
        .unwrap();
    println!("{min_x} {max_x}");
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

    let max = 4000000;
    let mut points = BTreeSet::new();
    let mut last = String::new();
    for (i, (s_x, s_y, _)) in bs
        .iter()
        .map(|(s, b)| ((s, b), manhattan_distance((s.x, s.y), (b.x, b.y))))
        .map(|((Sensor { x, y }, _), size)| (*x, *y, size))
        .enumerate()
    {
        let p = format!("{:.3?}", (i as f64 / (bs.len() - 1) as f64) * 100.0);
        if last != p {
            println!("{p}%");
            last = p;
        }
        for (b_x, b_y, _) in bs
            .iter()
            .map(|(s, b)| ((s, b), manhattan_distance((s.x, s.y), (b.x, b.y))))
            .map(|((Sensor { x, y }, _), size)| (*x, *y, size))
        {
            if s_x == b_x && s_y == b_y {
                continue;
            }
            let (d, m) = create_line((s_x, s_y), (b_x, b_y));
            'cringe: for x in s_x.min(b_x)..s_x.max(b_x) {
                let y = d * x + m;
                let mut clean = true;
                for (c, size_c) in bs
                    .iter()
                    .map(|(s, b)| (s, manhattan_distance((s.x, s.y), (b.x, b.y))))
                    .map(|(Sensor { x, y }, size)| ((*x, *y), size))
                {
                    clean = clean
                        && x >= 0
                        && x <= max
                        && y >= 0
                        && y <= max
                        && manhattan_distance((x, y), c) > size_c;
                    if !clean {
                        continue 'cringe;
                    }
                }
                if clean {
                    points.insert((x, y));
                }
            }
        }
    }
    println!("{points:?}");
    points.into_iter().for_each(|x| {
        let (x, y) = x;
        let ans = BigInt::from(x) * BigInt::from(4000000) + BigInt::from(y);
        println!("{ans:?}");
    });
    0.into()
    //let (x, y) = points.pop_first().unwrap();
    //BigInt::from(x) * BigInt::from(4000000) + BigInt::from(y)
}
