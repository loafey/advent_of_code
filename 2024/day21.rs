#![allow(static_mut_refs)]
use std::{marker::PhantomData, sync::LazyLock};

use pathfinding::prelude::dijkstra;
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

fn alike(b: &[char]) -> usize {
    let mut sum = 0;
    for (a, b) in unsafe { &CODE }.iter().zip(b) {
        if a == b {
            sum += 1;
        } else {
            break;
        }
    }
    unsafe { &CODE }.len() - sum
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    proxy: [char; 26],
    goal: Vec<char>,
}

fn recur(
    limit: usize,
    a: usize,
    b: usize,
    moves: &mut FxHashSet<(State, usize)>,
    proxy: &[char; 26],
    goal: &Vec<char>,
) {
    let code = unsafe { &CODE };
    macro_rules! m1 {
        ($layer:expr, $c:expr) => {{
            moves.insert((
                State {
                    proxy: {
                        let mut copy = *proxy;
                        copy[$layer] = $c;
                        copy
                    },
                    goal: (goal).clone(),
                },
                alike(&*goal),
            ));
        }};
    }
    if b == limit {
        match (proxy[a], proxy[b]) {
            ('<', '8') => m1!(limit, '7'),
            ('<', '9') => m1!(limit, '8'),
            ('<', '5') => m1!(limit, '4'),
            ('<', '6') => m1!(limit, '5'),
            ('<', '2') => m1!(limit, '1'),
            ('<', '3') => m1!(limit, '2'),
            ('<', 'A') => m1!(limit, '0'),
            ('>', '7') => m1!(limit, '8'),
            ('>', '8') => m1!(limit, '9'),
            ('>', '4') => m1!(limit, '5'),
            ('>', '5') => m1!(limit, '6'),
            ('>', '1') => m1!(limit, '2'),
            ('>', '2') => m1!(limit, '3'),
            ('>', '0') => m1!(limit, 'A'),
            ('^', '4') => m1!(limit, '7'),
            ('^', '5') => m1!(limit, '8'),
            ('^', '6') => m1!(limit, '9'),
            ('^', '1') => m1!(limit, '4'),
            ('^', '2') => m1!(limit, '5'),
            ('^', '3') => m1!(limit, '6'),
            ('^', '0') => m1!(limit, '2'),
            ('^', 'A') => m1!(limit, '3'),
            ('v', '7') => m1!(limit, '4'),
            ('v', '8') => m1!(limit, '5'),
            ('v', '9') => m1!(limit, '6'),
            ('v', '4') => m1!(limit, '1'),
            ('v', '5') => m1!(limit, '2'),
            ('v', '6') => m1!(limit, '3'),
            ('v', '2') => m1!(limit, '0'),
            ('v', '3') => m1!(limit, 'A'),
            ('A', x) => {
                let mut ng = goal.clone();
                ng.push(x);
                if ng.len() > 4
                    || !match ng.len() {
                        1 => ng[0] == code[0],
                        2 => ng[0] == code[0] && ng[1] == code[1],
                        3 => ng[0] == code[0] && ng[1] == code[1] && ng[2] == code[2],
                        4 => {
                            ng[0] == code[0]
                                && ng[1] == code[1]
                                && ng[2] == code[2]
                                && ng[3] == code[3]
                        }
                        _ => panic!(),
                    }
                {
                    return;
                };
                let alike = alike(&ng);
                moves.insert((
                    State {
                        proxy: *proxy,
                        goal: ng,
                    },
                    alike,
                ));
            }
            _ => {}
        }
    } else {
        match (proxy[a], proxy[b]) {
            ('<', 'v') => m1!(b, '<'),
            ('<', '>') => m1!(b, 'v'),
            ('<', 'A') => m1!(b, '^'),
            ('>', '<') => m1!(b, 'v'),
            ('>', 'v') => m1!(b, '>'),
            ('>', '^') => m1!(b, 'A'),
            ('^', 'v') => m1!(b, '^'),
            ('^', '>') => m1!(b, 'A'),
            ('v', '^') => m1!(b, 'v'),
            ('v', 'A') => m1!(b, '>'),
            ('A', _) => {
                recur(limit, a + 1, b + 1, moves, proxy, goal);
            }
            _ => {}
        };
    }
}

fn movy(limit: usize, state: State) -> FxHashSet<(State, usize)> {
    let State { proxy, goal } = state.clone();
    let mut moves: FxHashSet<(_, usize)> = FxHashSet::default();
    moves.extend(DPAD.get(&proxy[0]).unwrap().iter().map(|(_, c)| {
        (
            State {
                proxy: {
                    let mut copy = proxy;
                    copy[0] = *c;
                    copy
                },
                goal: goal.clone(),
            },
            alike(&goal),
        )
    }));

    recur(limit, 0, 1, &mut moves, &proxy, &goal);
    // println!("{}", moves.len());
    moves
}

static mut CODE: Vec<char> = Vec::new();
static DPAD: LazyLock<FxHashMap<char, Vec<(Direction, char)>>> = LazyLock::new(dpad);
fn solve(limit: usize) -> usize {
    let codes = include_str!("../inputs/2024/day21.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // let mut posses = ['A'; 5];
    codes
        .into_iter()
        .map(|code| {
            unsafe { CODE = code };
            // let mut pos = posses[0];
            // print!("{}: ", code.iter().collect::<String>());
            let mut sum = 0;
            let mut nums = Vec::new();
            for keypad_goal in unsafe { &CODE } {
                if keypad_goal.is_numeric() {
                    nums.push(*keypad_goal);
                }
            }
            let nums = nums.iter().collect::<String>().parse::<usize>().unwrap();
            let state = State {
                proxy: ['A'; 26],
                goal: Vec::new(),
            };

            let res = dijkstra(
                &state,
                |s| movy(limit, s.clone()),
                |State { goal, .. }| goal == unsafe { &CODE },
            );
            if let Some(res) = res {
                let res = res.0;
                println!("{} * {nums} = {}", res.len() - 1, (res.len() - 1) * nums);
                sum += (res.len() - 1) * nums;
            }
            sum
        })
        .sum()
}

pub fn part1() -> usize {
    solve(2)
}
pub fn part2() -> usize {
    let mut last = std::time::Duration::default();
    for size in 25..=25 {
        let time = std::time::Instant::now();
        let ans = solve(size);
        let t = time.elapsed();
        println!(
            "{}:\t{ans}, took {:?} ({}%)",
            size,
            t,
            ((t.as_secs_f64() / last.as_secs_f64()) * 100.0) as usize
        );
        last = t;
        // ans
    }

    0
}
