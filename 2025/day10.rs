use pathfinding::directed::astar::astar;
use rayon::prelude::*;
use std::{collections::HashMap, io::Write, path::PathBuf};

#[derive(Debug)]
struct Machine {
    indicator_lights: Vec<bool>,
    button_wiring: Vec<Vec<usize>>,
    voltage_req: Vec<u16>,
}

fn input() -> Vec<Machine> {
    include_str!("../inputs/2025/day10.input")
        .lines()
        .map(|s| {
            let mut p = s.split_whitespace();
            let lights = p.next().unwrap();
            let mut button_wiring = Vec::new();
            let mut voltage_req: Option<&'static str> = None;
            for str in p {
                if str.starts_with('{') {
                    voltage_req = Some(str);
                    break;
                }
                button_wiring.push(str);
            }
            let volt = voltage_req.unwrap();

            Machine {
                indicator_lights: lights[1..lights.len() - 1]
                    .chars()
                    .map(|c| c == '#')
                    .collect(),
                button_wiring: button_wiring
                    .into_iter()
                    .map(|s| {
                        s[1..s.len() - 1]
                            .split(',')
                            .map(|s| s.parse().unwrap())
                            .collect()
                    })
                    .collect(),
                voltage_req: volt[1..volt.len() - 1]
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}

#[memoize::memoize]
fn turn_on(state: Vec<bool>, inputs: Vec<Vec<usize>>, goal: Vec<bool>, depth: usize) -> usize {
    if depth == 0 {
        return 100;
    }
    let mut new_states = Vec::new();
    for i in &inputs {
        let mut new_state = state.clone();
        for i in i {
            new_state[*i] = !new_state[*i];
        }
        if new_state == goal {
            return 1;
        }
        new_states.push(new_state);
    }
    let mut min = usize::MAX;
    for new_state in new_states {
        let val = turn_on(new_state, inputs.clone(), goal.clone(), depth - 1);
        min = min.min(val);
    }
    1 + min
}

fn jolt_me(
    state: Vec<u64>,
    inputs: Vec<Vec<usize>>,
    goal: Vec<u64>,
    depth: usize,
    cache: &mut HashMap<(Vec<u64>, Vec<Vec<usize>>), usize>,
) -> usize {
    if let Some(cached) = cache.get(&(state.clone(), inputs.clone())) {
        return *cached;
    }
    fn inner(
        state: Vec<u64>,
        inputs: Vec<Vec<usize>>,
        goal: Vec<u64>,
        depth: usize,
        cache: &mut HashMap<(Vec<u64>, Vec<Vec<usize>>), usize>,
    ) -> usize {
        if depth == 0 {
            return 1000;
        }
        let mut new_states = Vec::new();
        'outer: for i in &inputs {
            let mut new_state = state.clone();
            for i in i {
                new_state[*i] += 1;
            }
            if new_state == goal {
                return 1;
            }
            for (a, b) in new_state.iter().zip(goal.iter()) {
                if a > b {
                    continue 'outer;
                }
            }
            new_states.push(new_state);
        }
        if new_states.is_empty() {
            return 1000;
        }
        let mut min = usize::MAX;
        for new_state in new_states {
            let val = jolt_me(new_state, inputs.clone(), goal.clone(), depth - 1, cache);
            min = min.min(val);
        }
        1 + min
    }
    let result = inner(state.clone(), inputs.clone(), goal, depth, cache);
    cache.insert((state, inputs), result);
    result
}

fn astar_me(state: u128, inputs: Vec<u128>, goal: u128) -> u128 {
    astar(
        &state,
        |s| {
            let mut new = Vec::new();
            for i in &inputs {
                let mut new_state = *s;
                new_state += *i;

                let bad = parts(new_state)
                    .iter()
                    .zip(parts(goal).iter())
                    .any(|(a, b)| a > b);
                if !bad {
                    new.push((new_state, 1))
                }
            }
            new
        },
        |p| {
            (parts(*p)
                .iter()
                .zip(parts(goal).iter())
                .map(|(a, b)| (*a as u128, *b as u128))
                .map(|(a, b)| if a > b { 1000 } else { (b - a).pow(2) })
                .sum::<u128>() as f64)
                .sqrt() as u128
        },
        |s| s == &goal,
    )
    .map(|a| a.1)
    .unwrap_or_default()
}

pub fn part1() -> usize {
    input()
        .into_iter()
        .map(|i| {
            let state = vec![false; i.indicator_lights.len()];
            turn_on(state, i.button_wiring, i.indicator_lights, 10)
        })
        .sum()
}

fn vec_to_num(nums: Vec<u16>) -> u128 {
    let mut result = 0;
    for (i, num) in nums.into_iter().enumerate() {
        result += num as u128 * (1000u128.pow(i as u32))
    }
    result
}

fn vec_to_adder(nums: Vec<u16>) -> u128 {
    nums.into_iter().map(|num| 1000u128.pow(num as u32)).sum()
}

fn parts(num: u128) -> [u16; 10] {
    let mut result = [0u16; 10];
    for i in 0..10 {
        result[i] = ((num % 1000u128.pow(i as u32 + 1)) / 1000u128.pow(i as u32)) as u16
    }
    result
}

pub fn part2() -> u128 {
    let input = input();
    input
        .into_iter()
        .enumerate()
        .par_bridge()
        .map(|(c, i)| {
            let cache = format!("cache/2025_10_p2_{c}");
            let wiring = i
                .button_wiring
                .into_iter()
                .map(|v| v.into_iter().map(|a| a as u16).collect::<Vec<_>>())
                .map(vec_to_adder)
                .collect();
            let string_start = format!("{c}: {}D - ", i.voltage_req.len() as u128);
            let goal = vec_to_num(i.voltage_req);
            if PathBuf::from(&cache).exists() {
                println!("{string_start}✅...");
                return std::fs::read_to_string(&cache).unwrap().parse().unwrap();
            }
            println!("{string_start}❗️...");
            let ans = astar_me(0, wiring, goal);
            if PathBuf::from("cache").exists() {
                std::fs::File::create(&cache)
                    .unwrap()
                    .write_all(format!("{ans}").as_bytes())
                    .unwrap();
            }
            ans
        })
        .sum()
}
