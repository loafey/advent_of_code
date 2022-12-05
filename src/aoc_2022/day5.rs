use crate::utils::{bset_pop_top, bset_push_top, parse};
use std::collections::{BTreeMap, VecDeque};

fn parse_input(func: fn(&mut VecDeque<char>, char)) -> String {
    let mut split = include_str!("input/day5.input").split("\n\n");
    let top = split.next().unwrap();
    let bottom = split.next().unwrap();
    let mut stacks = {
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
            .for_each(|(i, c)| bset_push_top(&mut map, i, c));
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
            let mut temp_stack = VecDeque::new();
            for _ in 0..amount {
                let moved = bset_pop_top(&mut stacks, &from);
                func(&mut temp_stack, moved);
            }
            stacks.get_mut(&to).unwrap().extend(temp_stack.into_iter());
        });

    stacks
        .into_iter()
        .filter_map(|(_, mut v)| v.pop())
        .collect()
}

pub fn part1() -> String {
    parse_input(VecDeque::push_back)
}

pub fn part2() -> String {
    parse_input(VecDeque::push_front)
}
