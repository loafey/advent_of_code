#![allow(unused)]
use std::{
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    hash::Hash,
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
