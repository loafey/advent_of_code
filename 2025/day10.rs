use pathfinding::directed::astar::astar;
use rayon::prelude::*;
use std::{collections::HashMap, io::Write, path::PathBuf};

#[derive(Debug)]
struct Machine {
    indicator_lights: Vec<bool>,
    button_wiring: Vec<Vec<usize>>,
    voltage_req: Vec<u64>,
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

fn astar_me(state: Vec<u64>, inputs: Vec<Vec<usize>>, goal: Vec<u64>) -> u64 {
    astar(
        &state,
        |s| {
            let mut new = Vec::new();
            for i in &inputs {
                let mut new_state = s.clone();
                for i in i {
                    new_state[*i] += 1;
                }
                let bad = new_state.iter().zip(goal.iter()).any(|(a, b)| a > b);
                if !bad {
                    new.push((new_state, 1))
                }
            }
            new
        },
        |p| {
            (p.iter()
                .zip(goal.iter())
                .map(|(a, b)| if a > b { 100000000 } else { (b - a).pow(2) })
                .sum::<u64>() as f64)
                .sqrt() as u64
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

pub fn part2() -> u64 {
    let input = input();
    input
        .into_iter()
        .enumerate()
        // .par_bridge()
        .map(|(c, i)| {
            let cache = format!("cache/2025_10_p2_{c}");
            if PathBuf::from(&cache).exists() {
                return std::fs::read_to_string(&cache).unwrap().parse().unwrap();
            }
            println!("{c}: {}D", i.voltage_req.len());
            let state = vec![0; i.voltage_req.len()];
            let ans = astar_me(state, i.button_wiring, i.voltage_req);
            println!("   got {ans} clicks");
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
