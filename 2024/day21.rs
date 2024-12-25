#![allow(static_mut_refs)]

// Big shoutout to: https://www.reddit.com/r/adventofcode/comments/1hjx0x4/2024_day_21_quick_tutorial_to_solve_part_2_in/
// I couldnt do it on my own, but i wanted to finish all stars :(

use pathfinding::prelude::astar_bag_collect;
use rustc_hash::{FxHashMap, FxHashSet};
use utils::Direction::{self, *};

fn dpad() -> FxHashMap<char, Vec<(Direction, char)>> {
    let mut map = FxHashMap::default();
    map.insert('^', vec![(Down, 'v'), (Right, 'A')]);
    map.insert('A', vec![(Down, '>'), (Left, '^')]);
    map.insert('<', vec![(Right, 'v')]);
    map.insert('v', vec![(Up, '^'), (Left, '<'), (Right, '>')]);
    map.insert('>', vec![(Up, 'A'), (Left, 'v')]);
    map
}

fn keypad() -> FxHashMap<char, Vec<(Direction, char)>> {
    let mut map = FxHashMap::default();
    map.insert('A', vec![(Up, '3'), (Left, '0')]);
    map.insert('0', vec![(Up, '2'), (Right, 'A')]);
    map.insert('1', vec![(Up, '4'), (Right, '2')]);
    map.insert('2', vec![(Up, '5'), (Down, '0'), (Left, '1'), (Right, '3')]);
    map.insert('3', vec![(Up, '6'), (Down, 'A'), (Left, '2')]);
    map.insert('4', vec![(Up, '7'), (Down, '1'), (Right, '5')]);
    map.insert('5', vec![(Up, '8'), (Down, '2'), (Left, '4'), (Right, '6')]);
    map.insert('6', vec![(Up, '9'), (Down, '3'), (Left, '5')]);
    map.insert('7', vec![(Down, '4'), (Right, '8')]);
    map.insert('8', vec![(Down, '5'), (Left, '7'), (Right, '9')]);
    map.insert('9', vec![(Down, '6'), (Left, '8')]);
    map
}

pub fn solve(len: usize) -> usize {
    let keypad = keypad();
    let dpad = dpad();
    let mut paths: FxHashMap<char, FxHashMap<char, FxHashSet<String>>> = FxHashMap::default();

    macro_rules! inps {
        ($pad:expr, $a:expr, $b:expr) => {
            let sols = astar_bag_collect(
                &(Up, *$a),
                |(_, a)| {
                    $pad.get(a)
                        .cloned()
                        .unwrap_or_default()
                        .into_iter()
                        .map(|a| (a, 1))
                        .collect::<Vec<_>>()
                },
                |_| 0,
                |(_, a)| a == $b,
            );
            if let Some((sols, _)) = sols {
                let r = paths.entry(*$a).or_default().entry(*$b).or_default();
                for s in sols {
                    r.insert(
                        s.into_iter()
                            .skip(1)
                            .map(|(a, _)| match a {
                                Up => '^',
                                Down => 'v',
                                Left => '<',
                                Right => '>',
                            })
                            .collect(),
                    );
                }
            }
        };
    }

    for a in keypad.keys() {
        for b in keypad.keys() {
            inps!(keypad, a, b);
        }
    }
    for a in dpad.keys() {
        for b in dpad.keys() {
            inps!(dpad, a, b);
        }
    }

    fn build_path(
        map: &FxHashMap<char, FxHashMap<char, FxHashSet<String>>>,
        keys: &str,
        a: char,
        cur_path: String,
        result: &mut Vec<String>,
    ) {
        if keys.is_empty() {
            result.push(cur_path);
            return;
        }
        let b = keys.chars().next().unwrap();
        for p in &map[&a][&b] {
            let mut cur_path = cur_path.clone();
            cur_path += p;
            cur_path += "A";
            build_path(map, &keys[1..], b, cur_path, result);
        }
    }

    static mut MAP: Option<FxHashMap<char, FxHashMap<char, FxHashSet<String>>>> = None;
    unsafe { MAP = Some(paths) };

    #[memoize::memoize]
    fn shortest_path(keys: String, depth: usize) -> usize {
        if depth == 0 {
            return keys.len();
        }

        let mut total = 0;
        let splat = keys.split('A').collect::<Vec<_>>();
        let len = splat.len();
        for (i, p) in splat.into_iter().enumerate() {
            let p = if i + 1 == len {
                p.to_string()
            } else {
                format!("{p}A")
            };
            let mut sub_seqs = Vec::new();
            build_path(
                unsafe { MAP.as_ref().unwrap() },
                &p,
                'A',
                String::new(),
                &mut sub_seqs,
            );
            let min = sub_seqs
                .into_iter()
                .map(|sub_seq| shortest_path(sub_seq, depth - 1))
                .min()
                .unwrap_or_default();
            total += min;
        }

        total
    }

    let mut tot = 0;
    for c in include_str!("../inputs/2024/day21.input")
        .lines()
        .filter(|s| !s.is_empty())
    {
        let num = c
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        let mut p = Vec::new();
        build_path(
            unsafe { MAP.as_ref().unwrap() },
            c,
            'A',
            String::new(),
            &mut p,
        );
        tot += num
            * p.into_iter()
                .map(|p| shortest_path(p, len))
                .min()
                .unwrap_or_default();
    }
    tot
}

pub fn part1() -> usize {
    solve(2)
}
pub fn part2() -> usize {
    solve(25)
}
