#![allow(unused)]
use std::{
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    hash::Hash,
    num::Wrapping,
    path::Path,
    process::Output,
    str::FromStr,
};

use chrono::format::Item;

pub trait SliceTools<T> {
    fn diff(&self, rhs: &[T]) -> Option<usize>;
    fn index_of(&self, rhs: &T) -> Option<usize>;
}
impl<T: PartialEq> SliceTools<T> for &[T] {
    fn diff(&self, rhs: &[T]) -> Option<usize> {
        if self.len() != rhs.len() {
            return None;
        }

        Some(
            self.iter()
                .zip(rhs.iter())
                .map(|(a, b)| if a != b { 1 } else { 0 })
                .sum(),
        )
    }

    fn index_of(&self, rhs: &T) -> Option<usize> {
        self.iter().position(|b| b == rhs)
    }
}
impl<T: PartialEq> SliceTools<T> for Vec<T> {
    fn diff(&self, rhs: &[T]) -> Option<usize> {
        if self.len() != rhs.len() {
            return None;
        }

        Some(
            self.iter()
                .zip(rhs.iter())
                .map(|(a, b)| if a != b { 1 } else { 0 })
                .sum(),
        )
    }
    fn index_of(&self, rhs: &T) -> Option<usize> {
        self.iter().position(|b| b == rhs)
    }
}

pub trait MatrixTools {
    fn transpose(self) -> Self;
    fn invert(self) -> Self;
    fn rotate(self) -> Self;
}
impl<T: Clone> MatrixTools for Vec<Vec<T>> {
    fn transpose(self) -> Self {
        if self.is_empty() {
            return Vec::new();
        }

        let mut nv = vec![Vec::new(); self[0].len()];
        for r in self.into_iter() {
            for (x, r) in r.into_iter().enumerate() {
                nv[x].push(r);
            }
        }

        nv
    }

    fn invert(self) -> Self {
        self.into_iter()
            .map(|mut r| {
                r.reverse();
                r
            })
            .collect()
    }

    fn rotate(self) -> Self {
        self.transpose().invert()
    }
}

pub trait NumTupleExt<T> {
    fn manhattan_distance(&self, other: &Self) -> T;
}
impl NumTupleExt<usize> for (usize, usize) {
    fn manhattan_distance(&self, other: &Self) -> usize {
        (self.0.max(other.0) - self.0.min(other.0)) + (self.1.max(other.1) - self.1.min(other.1))
    }
}
impl NumTupleExt<isize> for (isize, isize) {
    fn manhattan_distance(&self, other: &Self) -> isize {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

pub fn parse_next<'l, T: FromStr>(iter: &mut impl Iterator<Item = &'l str>) -> T {
    let s = iter.next();
    s.and_then(|s| s.parse::<T>().ok())
        .unwrap_or_else(|| panic!("Failed to parse string: \"{s:?}\""))
}

pub fn parse<F: FromStr + Debug>(s: &str) -> F {
    s.parse::<F>().ok().unwrap()
}

pub fn bset_pop_top<K: Eq + Ord, V>(map: &mut BTreeMap<K, Vec<V>>, key: &K) -> V {
    map.get_mut(key).unwrap().pop().unwrap()
}
pub fn bset_push_top<K: Eq + Ord, V>(map: &mut BTreeMap<K, Vec<V>>, key: K, val: V) {
    if let Some(vec) = map.get_mut(&key) {
        vec.push(val)
    } else {
        map.insert(key, vec![val]);
    }
}

pub fn hmap_insert_vec<K: Eq + Hash, V>(map: &mut HashMap<K, Vec<V>>, key: K, mut val: Vec<V>) {
    if let Some(vec) = map.get_mut(&key) {
        vec.append(&mut val);
    } else {
        map.insert(key, val);
    }
}

pub fn hmap_insert<K: Eq + Hash, V>(map: &mut HashMap<K, Vec<V>>, key: K, val: V) {
    if let Some(vec) = map.get_mut(&key) {
        vec.push(val)
    } else {
        map.insert(key, vec![val]);
    }
}

pub fn flip<A, B, C, F1: Fn(A, B) -> C + 'static>(f: F1) -> Box<dyn Fn(B, A) -> C> {
    Box::new(move |b: B, a: A| f(a, b))
}

pub fn ascii_4_art_to_string(a: &[Vec<char>; 6], gap: usize) -> String {
    const CHAR_LENGTH: usize = 4;

    let mut i = 0;
    let mut res = String::new();

    while i <= a[0].len() - gap {
        let cutout = [
            &a[0][i..i + CHAR_LENGTH],
            &a[1][i..i + CHAR_LENGTH],
            &a[2][i..i + CHAR_LENGTH],
            &a[3][i..i + CHAR_LENGTH],
            &a[4][i..i + CHAR_LENGTH],
            &a[5][i..i + CHAR_LENGTH],
        ];

        match cutout {
            [['#', '#', '#', '#'], ['#', '.', '.', '.'], ['#', '#', '#', '.'], ['#', '.', '.', '.'], ['#', '.', '.', '.'], ['#', '.', '.', '.']] => {
                res += "F"
            }
            [['#', '#', '#', '#'], ['#', '.', '.', '.'], ['#', '#', '#', '.'], ['#', '.', '.', '.'], ['#', '.', '.', '.'], ['#', '#', '#', '#']] => {
                res += "E"
            }
            [['.', '#', '#', '.'], ['#', '.', '.', '#'], ['#', '.', '.', '.'], ['#', '.', '.', '.'], ['#', '.', '.', '#'], ['.', '#', '#', '.']] => {
                res += "C"
            }
            [['#', '#', '#', '#'], ['.', '.', '.', '#'], ['.', '.', '#', '.'], ['.', '#', '.', '.'], ['#', '.', '.', '.'], ['#', '#', '#', '#']] => {
                res += "Z"
            }
            [['#', '.', '.', '.'], ['#', '.', '.', '.'], ['#', '.', '.', '.'], ['#', '.', '.', '.'], ['#', '.', '.', '.'], ['#', '#', '#', '#']] => {
                res += "L"
            }
            [['#', '.', '.', '#'], ['#', '.', '.', '#'], ['#', '#', '#', '#'], ['#', '.', '.', '#'], ['#', '.', '.', '#'], ['#', '.', '.', '#']] => {
                res += "H"
            }
            _ => {
                cutout.iter().for_each(|c| println!("{c:?}"));
                panic!("Unknown char!");
            }
        };
        i += CHAR_LENGTH + gap;
    }

    res
}

pub fn load_string<P: AsRef<Path>>(p: P) -> String {
    std::fs::read_to_string(p).unwrap()
}
// trait Splat<A, B> {
//     type Output;
//     fn splat(self, f: fn(A) -> B) -> Output;
// }

// impl<A, B> Splat<A, B> for (A, A) {
//     type Output = (B, B);
//     fn splat(self, f: fn(A) -> B) -> Output {
//         (f(self.0), f(self.1))
//     }
// }

pub trait IteratorEvalExt<T> {
    /// Strictly evaluate the `Iterator` returning an owned variant.
    fn eval(self) -> std::vec::IntoIter<T>;
}
impl<A, T: Iterator<Item = A>> IteratorEvalExt<A> for T {
    fn eval(self) -> std::vec::IntoIter<A> {
        let mut v = Vec::new();
        for i in self {
            v.push(i);
        }
        v.into_iter()
    }
}

pub trait IntoHMap<K, V> {
    fn hmap(self) -> HashMap<K, V>;
}
impl<const N: usize, K: Hash + Eq, V> IntoHMap<K, V> for [(K, V); N] {
    fn hmap(self) -> HashMap<K, V> {
        self.into()
    }
}
pub trait IntoBMap<K, V> {
    fn bmap(self) -> BTreeMap<K, V>;
}
impl<const N: usize, K: Ord + Eq, V> IntoBMap<K, V> for [(K, V); N] {
    fn bmap(self) -> BTreeMap<K, V> {
        self.into()
    }
}

pub fn matrix_get<T: Copy>(
    y: usize,
    x: usize,
    ymod: isize,
    xmod: isize,
    inputs: &[Vec<T>],
) -> Option<T> {
    let Wrapping(x) = if xmod < 0 {
        Wrapping(x) - Wrapping(xmod.unsigned_abs())
    } else {
        Wrapping(x) + Wrapping(xmod.unsigned_abs())
    };
    let Wrapping(y) = if ymod < 0 {
        Wrapping(y) - Wrapping(ymod.unsigned_abs())
    } else {
        Wrapping(y) + Wrapping(ymod.unsigned_abs())
    };
    inputs.get(y)?.get(x).cloned()
}

pub mod bi_functors {
    pub trait BiFunctorExtExt<A, B> {
        fn splet(self, ab: fn(A) -> B) -> (B, B);
    }
    impl<A, B> BiFunctorExtExt<A, B> for (A, A) {
        fn splet(self, ab: fn(A) -> B) -> (B, B) {
            let (e1, e2) = self;
            (ab(e1), ab(e2))
        }
    }

    pub trait BiFunctorExt<A, B, C> {
        fn splot(self, ab: fn(A, B) -> C) -> C;
    }
    impl<A, B, C> BiFunctorExt<A, B, C> for (A, B) {
        fn splot(self, ab: fn(A, B) -> C) -> C {
            let (e1, e2) = self;
            ab(e1, e2)
        }
    }

    pub trait BiFunctor<A, B, C, D> {
        fn splat(self, a: fn(A) -> B, b: fn(C) -> D) -> (B, D);
        fn splut(self, ab: fn(A, C) -> (B, D)) -> (B, D);
    }
    impl<A, B, C, D> BiFunctor<A, B, C, D> for (A, C) {
        fn splat(self, a: fn(A) -> B, b: fn(C) -> D) -> (B, D) {
            let (e1, e2) = self;
            (a(e1), b(e2))
        }

        fn splut(self, ab: fn(A, C) -> (B, D)) -> (B, D) {
            let (e1, e2) = self;
            ab(e1, e2)
        }
    }
    //impl<A, D> BiFunctor<A, D, A, D> for (A, A) {
    //    fn splat(self, a: fn(A) -> D, b: fn(A) -> D) -> (D, D) {
    //        let (e1, e2) = self;
    //        (a(e1), b(e2))
    //    }
    //}
}

pub trait FoldDefault<A, B: Default> {
    fn fold_d(self, f: fn(B, A) -> B) -> B;
}
impl<A, B: Default, I: Iterator<Item = A>> FoldDefault<A, B> for I {
    fn fold_d(self, f: fn(B, A) -> B) -> B {
        self.fold(B::default(), f)
    }
}
pub trait ParseAndCollect {
    fn parse_and_collect<A: FromIterator<B>, B: FromStr>(self) -> A
    where
        <B as FromStr>::Err: Debug;
}
impl<'a, I: Iterator<Item = &'a str>> ParseAndCollect for I {
    fn parse_and_collect<A: FromIterator<B>, B: FromStr>(self) -> A
    where
        <B as FromStr>::Err: Debug,
    {
        self.map(|b| b.parse().unwrap()).collect::<A>()
    }
}

pub trait NumExt {
    fn lcm(self, other: &Self) -> Self;
    fn gcd(self, other: &Self) -> Self;
}
impl NumExt for usize {
    // yoinked from here https://docs.rs/num-integer/0.1.45/src/num_integer/lib.rs.html#828
    fn lcm(self, other: &Self) -> Self {
        if self == 0 && *other == 0 {
            return 0;
        }
        let gcd = self.gcd(other);
        self * (*other / gcd) //.abs()
    }

    // yoinked from https://docs.rs/num-integer/0.1.45/src/num_integer/lib.rs.html#459
    fn gcd(self, other: &Self) -> Self {
        let mut m = self;
        let mut n = *other;
        if m == 0 || n == 0 {
            return (m | n);
        }
        let shift = (m | n).trailing_zeros();
        if m == Self::min_value() || n == Self::min_value() {
            return (1 << shift);
        }
        m >>= m.trailing_zeros();
        n >>= n.trailing_zeros();
        while m != n {
            if m > n {
                m -= n;
                m >>= m.trailing_zeros();
            } else {
                n -= m;
                n >>= n.trailing_zeros();
            }
        }
        m << shift
    }
}
