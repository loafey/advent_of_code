use utils::load_string;
use utils::{bset_pop_top, bset_push_top, parse};
use std::collections::{BTreeMap, VecDeque};

fn parse_input(deque_func: fn(&mut VecDeque<char>, char)) -> String {
    let binding = load_string("inputs/2022/day5.input");
    let mut split = binding.split("\n\n");
    let (Some(top), Some(bottom)) = (split.next(), split.next()) else {
        unreachable!()
    };

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
            let [_, a, _, f, _, t] = &splat[..] else {
                unreachable!()
            };
            (parse::<usize>(a), parse::<usize>(f), parse::<usize>(t))
        })
        .for_each(|(amount, from, to)| {
            let mut temp_stack = VecDeque::new();
            (0..amount).for_each(|_| deque_func(&mut temp_stack, bset_pop_top(&mut stacks, &from)));
            stacks.get_mut(&to).unwrap().extend(temp_stack);
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
