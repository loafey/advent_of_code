#![allow(unused)]
use std::{
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    hash::Hash,
    str::FromStr,
};

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

pub struct Zipper<'l, T> {
    inner: &'l Vec<T>,
    index: usize,
}
impl<'l, T> From<&'l Vec<T>> for Zipper<'l, T> {
    fn from(value: &'l Vec<T>) -> Self {
        Self {
            inner: value,
            index: 0,
        }
    }
}
impl<'l, T> Iterator for Zipper<'l, T> {
    type Item = (&'l [T], &'l T, &'l [T]);

    fn next(&mut self) -> Option<Self::Item> {
        let mut middle = self.inner.get(self.index)?;
        let left = &self.inner[(0..self.index)];
        let right = &self.inner[self.index + 1..self.inner.len()];
        self.index += 1;
        Some((left, middle, right))
    }
}

pub struct Zipper2D<'l, T> {
    inner: &'l Vec<Vec<T>>,
    index_x: usize,
    index_y: usize,
}
impl<'l, T> From<&'l Vec<Vec<T>>> for Zipper2D<'l, T> {
    fn from(value: &'l Vec<Vec<T>>) -> Self {
        Self {
            inner: value,
            index_x: 0,
            index_y: 0,
        }
    }
}
impl<'l, T> Iterator for Zipper2D<'l, T> {
    type Item = (&'l [T], &'l [T], &'l T, Vec<&'l T>, Vec<&'l T>);

    fn next(&mut self) -> Option<Self::Item> {
        let middle = self.inner.get(self.index_y)?.get(self.index_x)?;

        let left = &self.inner.get(self.index_y)?[0..self.index_x];
        let right =
            &self.inner.get(self.index_y)?[self.index_x + 1..self.inner.get(self.index_y)?.len()];

        let up = (self.inner[0..self.index_y].iter().map(|v| &v[self.index_x])).collect();
        let down = (self.inner[self.index_y + 1..self.inner.len()]
            .iter()
            .map(|v| &v[self.index_x]))
        .collect();

        self.index_x += 1;
        if self.index_x >= self.inner.get(self.index_y)?.len() {
            self.index_x = 0;
            self.index_y += 1;
        }
        Some((left, right, middle, up, down))
    }
}
