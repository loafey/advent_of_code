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

pub fn manhattan_distance(p1: (isize, isize), p2: (isize, isize)) -> isize {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
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
