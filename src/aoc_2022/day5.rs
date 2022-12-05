use crate::utils::parse;
use std::{
    collections::{BTreeMap, VecDeque},
    hash::Hash,
};

fn parse_input() -> (
    BTreeMap<usize, Vec<char>>,
    impl Iterator<Item = (usize, usize, usize)>,
) {
    let mut split = include_str!("input/day5.input").split("\n\n");
    let top = split.next().unwrap();
    let bottom = split.next().unwrap();
    let stacks = {
        const MOD: usize = 1;
        let mut map = BTreeMap::new();
        top.split('\n')
            .rev()
            .flat_map(|s| {
                s.chars()
                    .enumerate()
                    .filter(|(i, _)| (i + MOD) % 2 == 0)
                    .step_by(2)
                    .filter(|(_, c)| c.is_alphabetic())
                    .map(|(i, c)| ((i / 4) + 1, c))
            })
            .for_each(|(i, c)| push_top(&mut map, i, c));
        map
    };

    let iter = bottom.lines().map(|s| {
        let splat = s.split(' ').collect::<Vec<_>>();
        let [_, amount, _, from, _ ,to] = &splat[..] else {unreachable!()};
        (
            parse::<usize>(amount),
            parse::<usize>(from),
            parse::<usize>(to),
        )
    });

    (stacks, iter)
}

pub fn part1() -> String {
    let (mut stacks, iter) = parse_input();
    iter.for_each(|(amount, from, to)| {
        for _ in 0..amount {
            let moved = pop_top(&mut stacks, &from);
            push_top(&mut stacks, to, moved);
        }
    });
    stacks
        .into_iter()
        .filter_map(|(_, mut v)| v.pop())
        .collect()
}

fn pop_top<K: Eq + Hash + Ord, V>(map: &mut BTreeMap<K, Vec<V>>, key: &K) -> V {
    map.get_mut(key).unwrap().pop().unwrap()
}

fn push_top<K: Eq + Hash + Ord, V>(map: &mut BTreeMap<K, Vec<V>>, key: K, val: V) {
    if let Some(vec) = map.get_mut(&key) {
        vec.push(val)
    } else {
        map.insert(key, vec![val]);
    }
}

pub fn part2() -> String {
    let (mut stacks, iter) = parse_input();
    iter.for_each(|(amount, from, to)| {
        let mut temp_stack = VecDeque::new();
        for _ in 0..amount {
            let moved = pop_top(&mut stacks, &from);
            temp_stack.push_front(moved);
        }
        stacks.get_mut(&to).unwrap().extend(temp_stack.into_iter());
    });
    stacks
        .into_iter()
        .filter_map(|(_, mut v)| v.pop())
        .collect()
}
