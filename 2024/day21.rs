use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use pathfinding::prelude::{bfs, dijkstra};
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use utils::{
    Direction::{self, *},
    Memoize,
};

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

fn print_path(path: &[(Direction, char)]) {
    for (p, c) in path {
        if *c == 'P' {
            print!("A");
            continue;
        }
        match p {
            Direction::Up => print!("^"),
            Direction::Right => print!(">"),
            Direction::Down => print!("v"),
            Direction::Left => print!("<"),
        }
    }
    println!()
}

fn alike(a: &[char], b: &[char]) -> usize {
    let mut sum = 0;
    for (a, b) in a.iter().zip(b) {
        if a == b {
            sum += 1;
        } else {
            break;
        }
    }
    a.len() - sum
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State<const N: usize> {
    proxy: [char; N],
    goal: Vec<char>,
}

fn solve() -> usize {
    let codes = include_str!("../inputs/2024/day21.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let dpad = dpad();
    let keypad = keypad();

    // let mut posses = ['A'; 5];
    codes
        .into_par_iter()
        .map(|code| {
            // let mut pos = posses[0];
            // print!("{}: ", code.iter().collect::<String>());
            let mut sum = 0;
            let mut nums = Vec::new();
            for keypad_goal in &code {
                if keypad_goal.is_numeric() {
                    nums.push(*keypad_goal);
                }
            }
            let nums = nums.iter().collect::<String>().parse::<usize>().unwrap();
            let state = State {
                proxy: ['A'; 3],
                goal: Vec::new(),
            };

            let res = dijkstra(
                &state,
                |State { proxy, goal }| {
                    let mut moves: Vec<(_, usize)> = Vec::new();
                    moves.extend(dpad.get(&proxy[0]).unwrap().iter().map(|(d, c)| {
                        (
                            State {
                                proxy: [*c, proxy[1], proxy[2]],
                                goal: goal.clone(),
                            },
                            alike(&code, goal),
                        )
                    }));
                    macro_rules! m1 {
                        ($c:expr) => {
                            moves.push((
                                State {
                                    proxy: [proxy[0], $c, proxy[2]],
                                    goal: goal.clone(),
                                },
                                alike(&code, goal),
                            ))
                        };
                    }
                    macro_rules! m2 {
                        ($c:expr) => {
                            moves.push((
                                State {
                                    proxy: [proxy[0], proxy[1], $c],
                                    goal: goal.clone(),
                                },
                                alike(&code, goal),
                            ))
                        };
                    }
                    match (proxy[0], proxy[1]) {
                        ('<', 'v') => m1!('<'),
                        ('<', '>') => m1!('v'),
                        ('<', 'A') => m1!('^'),
                        ('>', '<') => m1!('v'),
                        ('>', 'v') => m1!('>'),
                        ('>', '^') => m1!('A'),
                        ('^', 'v') => m1!('^'),
                        ('^', '>') => m1!('A'),
                        ('v', '^') => m1!('v'),
                        ('v', 'A') => m1!('>'),
                        ('A', _) => match (proxy[1], proxy[2]) {
                            ('<', '8') => m2!('7'),
                            ('<', '9') => m2!('8'),
                            ('<', '5') => m2!('4'),
                            ('<', '6') => m2!('5'),
                            ('<', '2') => m2!('1'),
                            ('<', '3') => m2!('2'),
                            ('<', 'A') => m2!('0'),
                            ('>', '7') => m2!('8'),
                            ('>', '8') => m2!('9'),
                            ('>', '4') => m2!('5'),
                            ('>', '5') => m2!('6'),
                            ('>', '1') => m2!('2'),
                            ('>', '2') => m2!('3'),
                            ('>', '0') => m2!('A'),
                            ('^', '4') => m2!('7'),
                            ('^', '5') => m2!('8'),
                            ('^', '6') => m2!('9'),
                            ('^', '1') => m2!('4'),
                            ('^', '2') => m2!('5'),
                            ('^', '3') => m2!('6'),
                            ('^', '0') => m2!('2'),
                            ('^', 'A') => m2!('3'),
                            ('v', '7') => m2!('4'),
                            ('v', '8') => m2!('5'),
                            ('v', '9') => m2!('6'),
                            ('v', '4') => m2!('1'),
                            ('v', '5') => m2!('2'),
                            ('v', '6') => m2!('3'),
                            ('v', '2') => m2!('0'),
                            ('v', '3') => m2!('A'),
                            ('A', x) => {
                                let mut ng = goal.clone();
                                ng.push(x);
                                let alike = alike(&code, &ng);
                                moves.push((
                                    State {
                                        proxy: *proxy,
                                        goal: ng,
                                    },
                                    alike,
                                ))
                            }
                            _ => {}
                        },
                        _ => {}
                    };

                    moves
                },
                |State { goal, .. }| goal == &code,
            );
            if let Some(res) = res {
                let res = res.0;
                // println!("{} * {nums} = {}", res.len() - 1, (res.len() - 1) * nums);
                sum += (res.len() - 1) * nums;
            }
            sum
        })
        .sum()
}

pub fn part1() -> usize {
    let codes = include_str!("../inputs/2024/day21.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let dpad = dpad();
    let keypad = keypad();

    let mut lookup_table: BTreeMap<_, BTreeMap<_, _>> = BTreeMap::default();

    for t in ['<', '>', 'v', '^', 'A'] {
        for s in ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A'] {
            for g in ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A'] {
                let res = dijkstra(
                    &State {
                        proxy: [t, s, 'A'],
                        goal: Vec::new(),
                    },
                    |State { proxy, goal }| {
                        let mut moves: Vec<(_, usize)> = Vec::new();
                        moves.extend(dpad.get(&proxy[0]).unwrap().iter().map(|(d, c)| {
                            (
                                State {
                                    proxy: [*c, proxy[1], proxy[2]],
                                    goal: goal.clone(),
                                },
                                1,
                            )
                        }));
                        macro_rules! m1 {
                            ($c:expr) => {
                                moves.push((
                                    State {
                                        proxy: [proxy[0], $c, proxy[2]],
                                        goal: goal.clone(),
                                    },
                                    1,
                                ))
                            };
                        }
                        match (proxy[0], proxy[1]) {
                            ('<', '8') => m1!('7'),
                            ('<', '9') => m1!('8'),
                            ('<', '5') => m1!('4'),
                            ('<', '6') => m1!('5'),
                            ('<', '2') => m1!('1'),
                            ('<', '3') => m1!('2'),
                            ('<', 'A') => m1!('0'),
                            ('>', '7') => m1!('8'),
                            ('>', '8') => m1!('9'),
                            ('>', '4') => m1!('5'),
                            ('>', '5') => m1!('6'),
                            ('>', '1') => m1!('2'),
                            ('>', '2') => m1!('3'),
                            ('>', '0') => m1!('A'),
                            ('^', '4') => m1!('7'),
                            ('^', '5') => m1!('8'),
                            ('^', '6') => m1!('9'),
                            ('^', '1') => m1!('4'),
                            ('^', '2') => m1!('5'),
                            ('^', '3') => m1!('6'),
                            ('^', '0') => m1!('2'),
                            ('^', 'A') => m1!('3'),
                            ('v', '7') => m1!('4'),
                            ('v', '8') => m1!('5'),
                            ('v', '9') => m1!('6'),
                            ('v', '4') => m1!('1'),
                            ('v', '5') => m1!('2'),
                            ('v', '6') => m1!('3'),
                            ('v', '2') => m1!('0'),
                            ('v', '3') => m1!('A'),
                            _ => {}
                        };

                        moves
                    },
                    |State { proxy, .. }| proxy[1] == g,
                );
                if let Some(res) = res {
                    let res = res.0;
                    lookup_table
                        .entry((s, g))
                        .or_default()
                        .insert(t, (res.last().unwrap().proxy[0], (res.len() - 1) * 2));
                }
            }
        }
    }

    for t in ['<', '>', 'v', '^', 'A'] {
        for s in ['<', '>', 'v', '^', 'A'] {
            for g in ['<', '>', 'v', '^', 'A'] {
                let res = dijkstra(
                    &State {
                        proxy: [t, s, 'A'],
                        goal: Vec::new(),
                    },
                    |State { proxy, goal }| {
                        let mut moves: Vec<(_, usize)> = Vec::new();
                        moves.extend(dpad.get(&proxy[0]).unwrap().iter().map(|(d, c)| {
                            (
                                State {
                                    proxy: [*c, proxy[1], proxy[2]],
                                    goal: goal.clone(),
                                },
                                1,
                            )
                        }));
                        macro_rules! m1 {
                            ($c:expr) => {
                                moves.push((
                                    State {
                                        proxy: [proxy[0], $c, proxy[2]],
                                        goal: goal.clone(),
                                    },
                                    1,
                                ))
                            };
                        }
                        match (proxy[0], proxy[1]) {
                            ('<', 'v') => m1!('<'),
                            ('<', '>') => m1!('v'),
                            ('<', 'A') => m1!('^'),
                            ('>', '<') => m1!('v'),
                            ('>', 'v') => m1!('>'),
                            ('>', '^') => m1!('A'),
                            ('^', 'v') => m1!('^'),
                            ('^', '>') => m1!('A'),
                            ('v', '^') => m1!('v'),
                            ('v', 'A') => m1!('>'),
                            ('A', _) => {}
                            _ => {}
                        };

                        moves
                    },
                    |State { proxy, .. }| proxy[1] == g,
                );
                if let Some(res) = res {
                    let res = res.0;
                    lookup_table
                        .entry((s, g))
                        .or_default()
                        .insert(t, (res.last().unwrap().proxy[0], (res.len() - 1) * 2));
                }
            }
        }
    }

    // for ((from, to), map) in lookup_table {
    //     println!("From {from} to {to} costs:");
    //     for (top, (becomes, cost)) in map {
    //         println!("\tif top is {top}: {cost} (top becomes ({becomes}))");
    //     }
    // }
    for mut code in include_str!("../inputs/2024/day21.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect::<Vec<_>>())
    {
        let top = 'A';
        #[allow(clippy::never_loop)]
        for i in 0..=2 {
            let mut tot = [0, 0, 0, 0];
            let mut total_cost = 0;
            let mut new_code = Vec::new();
            let mut start = 'A';
            for (j, c) in code.into_iter().enumerate() {
                let cost = lookup_table.get(&(start, c)).unwrap();
                print!("{start} -> {c}: ");
                let (top2, cost2) = cost.get(&top).unwrap();
                println!("{cost2}");
                start = c;
                // top = *top2;
                new_code.push(*top2);

                total_cost += *cost2;
                tot[j] += *cost2 * i;
            }
            println!("- {} -\n--------------------", total_cost);
            // tot[i] = total_cost * (1 + i);
            code = new_code;
            println!("{tot:?}: {}", tot.iter().sum::<usize>());
        }
    }

    0
}
pub fn part2() -> usize {
    0
    // let codes = include_str!("../inputs/2024/day21.input")
    //     .lines()
    //     .filter(|s| !s.is_empty())
    //     .map(|s| s.chars().collect::<Vec<_>>())
    //     .collect::<Vec<_>>();
    // let dpad = dpad();
    // let keypad = keypad();

    // // let mut posses = ['A'; 5];
    // codes
    //     .into_iter()
    //     .map(|code| {
    //         let code = Memoize(Rc::new(code));
    //         // let mut pos = posses[0];
    //         // print!("{}: ", code.iter().collect::<String>());
    //         let mut sum = 0;
    //         let mut nums = Vec::new();
    //         for keypad_goal in &*code {
    //             if keypad_goal.is_numeric() {
    //                 nums.push(*keypad_goal);
    //             }
    //         }
    //         let nums = nums.iter().collect::<String>().parse::<usize>().unwrap();
    //         let state = State {
    //             proxy: ['A'; 25],
    //             goal: Vec::new(),
    //         };

    //         let res = dijkstra(
    //             &state,
    //             |State { proxy, goal }| {
    //                 let mut moves: Vec<(_, usize)> = Vec::new();
    //                 moves.extend(dpad.get(&proxy[0]).unwrap().iter().map(|(d, c)| {
    //                     (
    //                         State {
    //                             proxy: {
    //                                 let mut copy = *proxy;
    //                                 copy[0] = *c;
    //                                 copy
    //                             },
    //                             goal: goal.clone(),
    //                         },
    //                         alike(&code, goal),
    //                     )
    //                 }));

    //                 #[memoize::memoize]
    //                 fn recur(
    //                     a: usize,
    //                     b: usize,
    //                     moves: Memoize<RefCell<Vec<(State<25>, usize)>>>,
    //                     proxy: [char; 25],
    //                     goal: Vec<char>,
    //                     code: Memoize<Vec<char>>,
    //                 ) {
    //                     macro_rules! m1 {
    //                         ($layer:expr, $c:expr) => {
    //                             moves.borrow_mut().push((
    //                                 State {
    //                                     proxy: {
    //                                         let mut copy = proxy;
    //                                         copy[$layer] = $c;
    //                                         copy
    //                                     },
    //                                     goal: (goal).clone(),
    //                                 },
    //                                 alike(&code, (&*goal)),
    //                             ))
    //                         };
    //                     }
    //                     if b == 24 {
    //                         match (proxy[a], proxy[b]) {
    //                             ('<', '8') => m1!(2, '7'),
    //                             ('<', '9') => m1!(2, '8'),
    //                             ('<', '5') => m1!(2, '4'),
    //                             ('<', '6') => m1!(2, '5'),
    //                             ('<', '2') => m1!(2, '1'),
    //                             ('<', '3') => m1!(2, '2'),
    //                             ('<', 'A') => m1!(2, '0'),
    //                             ('>', '7') => m1!(2, '8'),
    //                             ('>', '8') => m1!(2, '9'),
    //                             ('>', '4') => m1!(2, '5'),
    //                             ('>', '5') => m1!(2, '6'),
    //                             ('>', '1') => m1!(2, '2'),
    //                             ('>', '2') => m1!(2, '3'),
    //                             ('>', '0') => m1!(2, 'A'),
    //                             ('^', '4') => m1!(2, '7'),
    //                             ('^', '5') => m1!(2, '8'),
    //                             ('^', '6') => m1!(2, '9'),
    //                             ('^', '1') => m1!(2, '4'),
    //                             ('^', '2') => m1!(2, '5'),
    //                             ('^', '3') => m1!(2, '6'),
    //                             ('^', '0') => m1!(2, '2'),
    //                             ('^', 'A') => m1!(2, '3'),
    //                             ('v', '7') => m1!(2, '4'),
    //                             ('v', '8') => m1!(2, '5'),
    //                             ('v', '9') => m1!(2, '6'),
    //                             ('v', '4') => m1!(2, '1'),
    //                             ('v', '5') => m1!(2, '2'),
    //                             ('v', '6') => m1!(2, '3'),
    //                             ('v', '2') => m1!(2, '0'),
    //                             ('v', '3') => m1!(2, 'A'),
    //                             ('A', x) => {
    //                                 let mut ng = goal.clone();
    //                                 ng.push(x);
    //                                 let alike = alike(&*code, &ng);
    //                                 moves.borrow_mut().push((State { proxy, goal: ng }, alike))
    //                             }
    //                             _ => {}
    //                         }
    //                     } else {
    //                         match (proxy[0], proxy[1]) {
    //                             ('<', 'v') => m1!(1, '<'),
    //                             ('<', '>') => m1!(1, 'v'),
    //                             ('<', 'A') => m1!(1, '^'),
    //                             ('>', '<') => m1!(1, 'v'),
    //                             ('>', 'v') => m1!(1, '>'),
    //                             ('>', '^') => m1!(1, 'A'),
    //                             ('^', 'v') => m1!(1, '^'),
    //                             ('^', '>') => m1!(1, 'A'),
    //                             ('v', '^') => m1!(1, 'v'),
    //                             ('v', 'A') => m1!(1, '>'),
    //                             ('A', _) => {
    //                                 recur(a + 1, b + 1, moves, proxy, goal, code);
    //                             }
    //                             _ => {}
    //                         };
    //                     }
    //                 }
    //                 let moves = Memoize(RefCell::new(moves).into());
    //                 recur(0, 1, moves.clone(), *proxy, goal.clone(), code.clone());
    //                 let mvb = moves.borrow();
    //                 mvb.clone()
    //             },
    //             |State { goal, .. }| goal == &*code,
    //         );
    //         if let Some(res) = res {
    //             let res = res.0;
    //             println!("{} * {nums} = {}", res.len() - 1, (res.len() - 1) * nums);
    //             sum += (res.len() - 1) * nums;
    //         }
    //         sum
    //     })
    //     .sum()
}
