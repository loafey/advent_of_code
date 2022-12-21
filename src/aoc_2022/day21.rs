use std::{
    collections::HashMap,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug)]
enum Monkey {
    Value(isize),
    Dependant(&'static str, fn(isize, isize) -> isize, &'static str),
}
impl Monkey {
    fn is_value(&self) -> bool {
        match self {
            Monkey::Value(_) => true,
            Monkey::Dependant(_, _, _) => false,
        }
    }
    fn assume(&self) -> isize {
        match self {
            Monkey::Value(i) => *i,
            Monkey::Dependant(_, _, _) => unreachable!(),
        }
    }
}

fn input() -> HashMap<&'static str, Monkey> {
    include_str!("input/day21.input")
        .lines()
        .map(|s| {
            let mut splat = s.split(|c| c == ':' || c == ' ').filter(|s| !s.is_empty());

            let name = splat.next().unwrap();
            let next = splat.next().unwrap();
            if let Ok(n) = next.parse::<isize>() {
                (name, Monkey::Value(n))
            } else {
                let e1 = next;
                let op = splat.next().unwrap();
                let e2 = splat.next().unwrap();
                let op = match op {
                    "+" => isize::add,
                    "-" => isize::sub,
                    "*" => isize::mul,
                    "/" => isize::div,
                    _ => unreachable!(),
                };

                (name, Monkey::Dependant(e1, op, e2))
            }
        })
        .collect()
}

pub fn part1() -> isize {
    let mut monkeys = input();
    let mut stack = vec!["root"];
    while !stack.is_empty() {
        let top = stack[stack.len() - 1];
        match monkeys.get(top).unwrap() {
            Monkey::Value(_) => {
                stack.pop();
            }
            Monkey::Dependant(e1, f, e2) => {
                if monkeys[e1].is_value() && monkeys[e2].is_value() {
                    *monkeys.get_mut(top).unwrap() =
                        Monkey::Value(f(monkeys[e1].assume(), monkeys[e2].assume()))
                } else {
                    if !monkeys[e1].is_value() {
                        stack.push(e1);
                    }
                    if !monkeys[e2].is_value() {
                        stack.push(e2);
                    }
                };
            }
        }
    }
    monkeys["root"].assume()
}

pub fn part2() -> i32 {
    0
}
