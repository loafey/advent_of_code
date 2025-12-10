use good_lp::{
    Expression, ProblemVariables, Solution, SolverModel, constraint, default_solver, variable,
    variables,
};
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

pub fn part1() -> usize {
    input()
        .into_iter()
        .map(|i| {
            let state = vec![false; i.indicator_lights.len()];
            turn_on(state, i.button_wiring, i.indicator_lights, 10)
        })
        .sum()
}

fn vec_to_num(nums: Vec<u16>) -> f64 {
    let mut result = 0.0;
    for (i, num) in nums.into_iter().enumerate() {
        result += num as f64 * (1000f64.powi(i as i32))
    }
    result
}

fn vec_to_adder(nums: Vec<u16>) -> f64 {
    nums.into_iter().map(|num| 1000f64.powi(num as i32)).sum()
}

fn parts(num: f64) -> [u16; 10] {
    let mut result = [0u16; 10];
    for i in 0..10 {
        result[i] = ((num % 1000f64.powi(i as i32 + 1)) / 1000f64.powi(i as i32)) as u16
    }
    result
}

pub fn part2() -> u64 {
    let input = input();
    let mut sum = 0;
    for problem in input {
        println!("{problem:?}");
        let answer = vec_to_num(problem.voltage_req.clone());

        let mut vars = ProblemVariables::new();
        let mut names = Vec::new();
        let mut char = 'a';
        for button in &problem.button_wiring {
            let value = vec_to_adder(button.clone().into_iter().map(|s| s as u16).collect());
            let variable = variable().min(0).name(format!("{char}")).integer();
            char = (char as u8 + 1) as char;
            names.push((vars.add(variable), value));
        }
        let cloned_names = names.clone();
        let mut formula = Expression::from(0);
        for (var, value) in names {
            formula += var * value;
        }
        // println!("solving: {formula:?} = {answer}");
        let mut solver = vars.minimise(formula.clone()).using(default_solver);
        solver.set_parameter("loglevel", "0");
        for (i, (var, value)) in cloned_names.iter().enumerate() {
            for button in &problem.button_wiring[i] {
                let req = problem.voltage_req[*button];
                solver = solver.with(constraint!(*var <= req));
                solver = solver.with(constraint!(*var >= 0));
            }
        }
        solver = solver.with(constraint!(formula == answer));
        let ans = solver.solve().unwrap();
        let my_output = cloned_names
            .iter()
            .map(|(v, val)| ans.value(*v) * *val)
            .sum::<f64>();
        println!(
            "🐈 {:?}: {} ({}) - {}",
            cloned_names
                .iter()
                .map(|(v, _)| ans.value(*v))
                .collect::<Vec<_>>(),
            my_output,
            if my_output == answer { "✅" } else { "❗️" },
            cloned_names.iter().map(|(v, _)| ans.value(*v)).sum::<f64>()
        );
        sum += cloned_names.iter().map(|(v, _)| ans.value(*v)).sum::<f64>() as u64;
    }
    sum
}
