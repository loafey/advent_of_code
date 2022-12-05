#![allow(unused)]
use std::{collections::BTreeMap, fmt::Debug, str::FromStr};

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

pub fn flip<A, B, C, F1: Fn(A, B) -> C + 'static>(f: F1) -> Box<dyn Fn(B, A) -> C> {
    Box::new(move |b: B, a: A| f(a, b))
}
