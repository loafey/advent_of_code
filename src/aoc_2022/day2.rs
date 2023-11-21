use crate::utils::load_string;
use std::ops::{BitXor, Neg, Shr};

#[derive(Clone, Copy)]
struct C(i32);

impl Shr for C {
    type Output = i32;
    fn shr(self, rhs: Self) -> Self::Output {
        let (C(c1), C(c2)) = (self, rhs);
        -rhs + match match (c2 - c1).abs() {
            0 => -1,
            1 => c1.max(c2),
            _ => c1.min(c2),
        } {
            x if c1 == x => 0,
            x if c2 == x => 6,
            _ => 3,
        }
    }
}

impl Neg for C {
    type Output = i32;
    fn neg(self) -> Self::Output {
        let C(c1) = self;
        c1 + 1
    }
}

impl BitXor for C {
    type Output = Self;
    fn bitxor(self, rhs: C) -> Self::Output {
        let (C(c1), C(c2)) = (self, rhs);
        C(match (c1, c2) {
            (1, 0) | (2, 2) => 0,
            (0, 2) | (2, 0) => 1,
            (0, 0) | (1, 2) => 2,
            _ => c1,
        })
    }
}

fn str_to_choice(s: &str) -> (C, C) {
    match s.as_bytes() {
        [a, b' ', b, ..] => (
            C(*a as i32 - 65),
            C(match *b >= 88 {
                true => *b as i32 - 88,
                _ => *b as i32 - 65,
            }),
        ),
        _ => unreachable!(),
    }
}

pub fn part1() -> i32 {
    load_string("inputs/2022/day2.input")
        .lines()
        .map(str_to_choice)
        .map(|(o, y)| o >> y)
        .sum()
}

pub fn part2() -> i32 {
    load_string("inputs/2022/day2.input")
        .lines()
        .map(str_to_choice)
        .map(|(o, y)| o >> (o ^ y))
        .sum()
}
