use std::collections::HashMap;

use utils::{load_string, SliceTools};

fn hash(s: &str) -> usize {
    let mut current_value = 0;
    s.chars().for_each(|a| {
        let h = a as u8 as usize;
        current_value += h;
        current_value *= 17;
        current_value %= 256;
    });
    current_value
}

#[derive(Eq)]
struct BoxValue<'l> {
    str: &'l str,
    value: usize,
}

impl PartialEq for BoxValue<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.str == other.str
    }
}

pub fn part1() -> usize {
    load_string("inputs/2023/day15.input")
        .trim()
        .split(',')
        .map(hash)
        .sum()
}
pub fn part2() -> usize {
    let mut map = (0..256).map(|i| (i, Vec::new())).collect::<HashMap<_, _>>();
    let binding = load_string("inputs/2023/day15.input");
    for s in binding.trim().split(',') {
        if s.contains('-')
            && let Some(s) = s.split('-').next()
        {
            let h = hash(s);
            let v = map.get_mut(&h).unwrap();
            if let Some(ind) = v.index_of(&BoxValue { str: s, value: 0 }) {
                v.remove(ind);
            }
        } else if let Some((s, n)) = s.split_once('=') {
            let h = hash(s);
            let n = n.parse::<usize>().unwrap();
            let v = map.get_mut(&h).unwrap();
            if let Some(i) = v.index_of(&BoxValue { str: s, value: 0 }) {
                v.remove(i);
                v.insert(i, BoxValue { str: s, value: n });
            } else {
                v.push(BoxValue { str: s, value: n });
            }
        } else {
            unreachable!();
        }
    }
    map.into_iter()
        .filter(|(_, v)| !v.is_empty())
        .map(|(i, v)| {
            v.into_iter()
                .enumerate()
                .map(|(slot, BoxValue { value, .. })| (1 + i) * (slot + 1) * value)
                .sum::<usize>()
        })
        .sum()
}
