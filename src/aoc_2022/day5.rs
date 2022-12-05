use std::{collections::HashMap, hash::Hash};

use crate::utils::parse;

pub fn part1() -> String {
    let mut split = include_str!("input/day5.input").split("\n\n");
    let top = split.next().unwrap();
    let bottom = split.next().unwrap();
    let mut stacks = {
        const MOD: usize = 1;
        let mut map: HashMap<usize, Vec<char>> = HashMap::new();
        top.split('\n')
            .rev()
            .map(|s| {
                s.chars()
                    .enumerate()
                    .filter(|(i, _)| (i + MOD) % 2 == 0)
                    .step_by(2)
                    .filter(|(_, c)| c.is_alphabetic())
                    .map(|(i, c)| ((i / 4) + 1, c))
                    .for_each(|(i, c)| push_top(&mut map, i, c))
            })
            .for_each(|_| {});

        map
    };
    bottom
        .lines()
        .map(|s| {
            let splat = s.split(' ').collect::<Vec<_>>();
            let [_, amount, _, from, _ ,to] = &splat[..] else {unreachable!()};
            (
                parse::<usize>(amount),
                parse::<usize>(from),
                parse::<usize>(to),
            )
        })
        .for_each(|(amount, from, to)| {
            for _ in 0..amount {
                let moved = pop_top(&mut stacks, &from);
                push_top(&mut stacks, to, moved);
            }
        });
    let mut picked_out = stacks.into_iter().collect::<Vec<_>>();
    picked_out.sort_by_key(|(i, _)| *i);
    picked_out
        .into_iter()
        .filter_map(|(_, mut v)| v.pop())
        .collect()
}

fn pop_top<K: Eq + Hash, V>(map: &mut HashMap<K, Vec<V>>, key: &K) -> V {
    map.get_mut(key).unwrap().pop().unwrap()
}

fn push_top<K: Eq + Hash, V>(map: &mut HashMap<K, Vec<V>>, key: K, val: V) {
    if let Some(vec) = map.get_mut(&key) {
        vec.push(val)
    } else {
        map.insert(key, vec![val]);
    }
}

pub fn part2() -> String {
    let mut split = include_str!("input/day5.input").split("\n\n");
    let top = split.next().unwrap();
    let bottom = split.next().unwrap();
    let mut stacks = {
        const MOD: usize = 1;
        let mut map: HashMap<usize, Vec<char>> = HashMap::new();
        top.split('\n')
            .rev()
            .map(|s| {
                s.chars()
                    .enumerate()
                    .filter(|(i, _)| (i + MOD) % 2 == 0)
                    .step_by(2)
                    .filter(|(_, c)| c.is_alphabetic())
                    .map(|(i, c)| ((i / 4) + 1, c))
                    .for_each(|(i, c)| push_top(&mut map, i, c))
            })
            .for_each(|_| {});

        map
    };
    bottom
        .lines()
        .map(|s| {
            let splat = s.split(' ').collect::<Vec<_>>();
            let [_, amount, _, from, _ ,to] = &splat[..] else {unreachable!()};
            (
                parse::<usize>(amount),
                parse::<usize>(from),
                parse::<usize>(to),
            )
        })
        .for_each(|(amount, from, to)| {
            let mut temp_stack = Vec::new();
            for _ in 0..amount {
                let moved = pop_top(&mut stacks, &from);
                temp_stack.push(moved);
            }
            temp_stack.reverse();
            stacks.get_mut(&to).unwrap().append(&mut temp_stack);
        });
    let mut picked_out = stacks.into_iter().collect::<Vec<_>>();
    picked_out.sort_by_key(|(i, _)| *i);
    picked_out
        .into_iter()
        .filter_map(|(_, mut v)| v.pop())
        .collect()
}
