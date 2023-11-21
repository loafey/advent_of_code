use crate::utils::load_string;
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
fn parse_input() -> Vec<(Sensor, Beacon)> {
    load_string("inputs/2022/day15.input")
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
        .collect::<Vec<_>>()
}

pub fn part1() -> usize {
    const Y: isize = 2000000;
    let bs = parse_input();
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
    let mut row = vec![Spot::Empty; (max_x - min_x) as usize];

    for (s, b) in bs.iter() {
        if b.y == Y {
            row[(b.x - min_x) as usize] = Spot::Beacon
        }
        if s.y == Y {
            row[(s.x - min_x) as usize] = Spot::Sensor
        }
    }

    bs.into_iter()
        .map(|(s, b)| ((s, b), manhattan_distance((s.x, s.y), (b.x, b.y))))
        .filter(|((s, _), size)| (Y <= s.y + size) && (Y >= s.y - size))
        .map(|((s, _), size)| (s, size))
        .for_each(|(Sensor { x: s_x, y: s_y }, size)| {
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
            let range = s_x - dif..=s_x + dif;
            for x in range {
                if row[(x - min_x) as usize] == Spot::Empty {
                    row[(x - min_x) as usize] = Spot::Occupied
                }
            }
        });

    row.into_iter().filter(|s| *s == Spot::Occupied).count()
}

pub fn part2() -> isize {
    let bs = parse_input();
    let max = 4000000;
    for y in 0..max {
        let mut x = 0;
        while x < max {
            if let Some((s, size)) = bs
                .iter()
                .map(|(Sensor { x, y }, b)| ((*x, *y), manhattan_distance((*x, *y), (b.x, b.y))))
                .find(|((s_x, s_y), size)| manhattan_distance((x, y), (*s_x, *s_y)) <= *size)
            {
                let dif = (y - s.1).abs();
                x = s.0 + size - dif;
            } else {
                return x * 4000000 + y;
            }
            x += 1;
        }
    }
    isize::MIN
}
