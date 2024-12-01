use std::ops::{Add, Sub};

use utils::load_string;

#[derive(Debug, Clone, Copy)]
enum Move {
    L(i32),
    R(i32),
    U(i32),
    D(i32),
}
impl Move {
    fn parse(s: &str) -> Vec<Move> {
        use Move::*;
        s.split(',')
            .map(|s| {
                let p = s[1..].parse().unwrap();
                match &s[..1] {
                    "L" => L(p),
                    "R" => R(p),
                    "U" => U(p),
                    "D" => D(p),
                    _ => unreachable!(),
                }
            })
            .collect()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos(i32, i32);
impl Add<(i32, i32)> for Pos {
    type Output = Pos;
    fn add(self, (l, r): (i32, i32)) -> Self::Output {
        Pos(self.0 + l, self.1 + r)
    }
}
impl Sub<(i32, i32)> for Pos {
    type Output = Pos;
    fn sub(self, (l, r): (i32, i32)) -> Self::Output {
        Pos(self.0 - l, self.1 - r)
    }
}
impl Pos {
    fn mov(mut self, mov: Move) -> Pos {
        match mov {
            Move::L(p) => self.0 -= p,
            Move::R(p) => self.0 += p,
            Move::U(p) => self.1 += p,
            Move::D(p) => self.1 -= p,
        };
        self
    }
    fn move_me(self, set: &[Move]) -> Vec<Range> {
        let mut v = vec![(self, self)];
        for s in set {
            let (_, m) = v[v.len() - 1];
            let n = m.mov(*s);
            v.push((m, n));
        }
        v
    }
    fn overlap(self, (r1, r2): Range) -> bool {
        if self == Pos(0, 0) {
            false
        } else if r1.0 == r2.0 {
            let min = r1.1.min(r2.1);
            let max = r1.1.max(r2.1);
            self.0 == r1.0 && (min..max).contains(&self.1)
        } else if r1.1 == r2.1 {
            let min = r1.0.min(r2.0);
            let max = r1.0.max(r2.0);
            self.1 == r1.1 && (min..max).contains(&self.0)
        } else {
            false
        }
    }
    fn overlaps(mut self, set: &[Move], check: &[Range]) -> Vec<Pos> {
        let mut overlaps = Vec::new();
        for s in set {
            let (limit, func): (_, fn(Self) -> Self) = match *s {
                Move::L(p) => (p, |s| s - (1, 0)),
                Move::R(p) => (p, |s| s + (1, 0)),
                Move::U(p) => (p, |s| s + (0, 1)),
                Move::D(p) => (p, |s| s - (0, 1)),
            };
            for _ in 0..limit {
                overlaps.extend(check.iter().filter_map(|r| {
                    if self.overlap(*r) {
                        Some(self)
                    } else {
                        None
                    }
                }));
                self = func(self);
            }
        }
        overlaps
    }
    fn manhattan(self, other: Pos) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}
type Range = (Pos, Pos);

pub fn part1() -> i32 {
    let s = load_string("inputs/2019/day3.input");
    let (moveset1, moveset2) = s.trim().split_once('\n').unwrap();
    let (moveset1, moveset2) = (Move::parse(moveset1), Move::parse(moveset2));
    let p = Pos(0, 0);
    let p1 = p.move_me(&moveset1);
    let p2 = p.overlaps(&moveset2, &p1);
    p2.into_iter()
        .map(|o| o.manhattan(p))
        .min()
        .unwrap_or_default()
}

pub fn part2() -> i32 {
    0
}
