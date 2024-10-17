//! Absolutely horrible string parsing and iterator lib!
//!
//!
//! Split a file in two, count the chars on either side, combine and return result.
//! ```rs
//! let inp = load_string("src/aoc_2023/day1.input");
//! (parse(&inp) / "\n\n")
//! << (
//!     |s| (s | "") - (0, |a, _| a + 1),
//!     |s| (s | "") - (0, |a, _| a + 1),
//! )
//! >> (|a, b| a + b)
//! ```
//!
//! Advent of Code day 1 part 1:
//! ```rs
//! let inp = load_string("src/aoc_2023/day1.input");
//! ((parse(&inp) | "\n\n") ^ parse ^ (|s| (!(s | '\n')) - (0, |a, b| a + b))) - (0, i64::max)
//! ```

use std::{
    ops::{BitOr, BitXor, Div, Neg, Not, Shl, Shr, Sub},
    str::pattern::Pattern,
};

pub struct ParseStep<'a, T: 'a>(Box<dyn Iterator<Item = T> + 'a>);
pub fn parse_step<'a, T: 'a, I: Iterator<Item = T> + 'a>(v: I) -> ParseStep<'a, T> {
    ParseStep(Box::new(v))
}
impl<T> Neg for ParseStep<'_, T> {
    type Output = Vec<T>;

    fn neg(self) -> Self::Output {
        self.0.collect()
    }
}
impl<'a, T, B: 'a, F: Fn(T) -> B + 'a> BitXor<F> for ParseStep<'a, T> {
    type Output = ParseStep<'a, B>;

    fn bitxor(self, rhs: F) -> Self::Output {
        parse_step(self.0.map(rhs))
    }
}

impl<'a, T, F: Fn(&T) -> bool + 'a> Div<F> for ParseStep<'a, T> {
    type Output = ParseStep<'a, T>;

    fn div(self, rhs: F) -> Self::Output {
        parse_step(self.0.filter(rhs))
    }
}

impl<'a, T, B: 'a, F: Fn(B, T) -> B> Sub<(B, F)> for ParseStep<'a, T> {
    type Output = B;

    fn sub(self, (b, f): (B, F)) -> Self::Output {
        self.0.fold(b, f)
    }
}
impl<'a> Not for ParseStep<'a, &'a str> {
    type Output = ParseStep<'a, i64>;

    fn not(self) -> Self::Output {
        parse_step(self.0.map(|s| s.parse().unwrap()))
    }
}

pub fn parse(p: &str) -> Parser<'_> {
    Parser(p)
}
pub struct Parser<'a>(&'a str);
impl<'a, P: Pattern + 'a> BitOr<P> for Parser<'a> {
    type Output = ParseStep<'a, &'a str>;

    fn bitor(self, rhs: P) -> Self::Output {
        let Parser(s) = self;
        let split = s.split(rhs);
        parse_step(split)
    }
}
impl<'a, P: Pattern + 'a> Div<P> for Parser<'a> {
    type Output = SplitOnce<Parser<'a>, Parser<'a>>;

    fn div(self, rhs: P) -> Self::Output {
        let Parser(s) = self;
        let (s1, s2) = s.split_once(rhs).unwrap();
        SplitOnce(parse(s1), parse(s2))
    }
}

#[derive(Debug)]
pub struct SplitOnce<A, C>(A, C);
impl<A, B, C, D> Shl<(fn(A) -> B, fn(C) -> D)> for SplitOnce<A, C> {
    type Output = SplitOnce<B, D>;

    fn shl(self, (ab, cd): (fn(A) -> B, fn(C) -> D)) -> Self::Output {
        let SplitOnce(a, c) = self;
        SplitOnce(ab(a), cd(c))
    }
}
impl<A, C, E> Shr<fn(A, C) -> E> for SplitOnce<A, C> {
    type Output = E;

    fn shr(self, rhs: fn(A, C) -> E) -> Self::Output {
        let SplitOnce(a, c) = self;
        rhs(a, c)
    }
}
