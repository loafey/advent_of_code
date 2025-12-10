use good_lp::{
    Expression, ProblemVariables, Solution, SolverModel, Variable, constraint, default_solver,
    variable,
};

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
        .map(|problem| {
            let mut vars = ProblemVariables::new();
            let mut names = Vec::new();
            let mut values: Vec<(_, Vec<Variable>)> = problem
                .voltage_req
                .clone()
                .into_iter()
                .map(|v| (v as i32, Vec::new()))
                .collect();
            let mut press_formula = Expression::from(0);
            for button in &problem.button_wiring {
                let variable = variable().min(0).integer();
                let variable = vars.add(variable);
                for b in button {
                    values[*b].1.push(variable);
                }
                names.push(variable);
                press_formula += variable;
            }
            let mut solver = vars.minimise(press_formula.clone()).using(default_solver);
            solver.set_parameter("loglevel", "0");

            for (max_val, vars) in values {
                let expr = vars.into_iter().sum::<Expression>();
                solver = solver.with(constraint!(expr == max_val));
            }

            let ans = solver.solve().unwrap();
            names.iter().map(|v| ans.value(*v)).sum::<f64>() as u64
        })
        .sum()
}
