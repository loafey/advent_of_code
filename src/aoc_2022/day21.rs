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
        matches!(self, Monkey::Value(_))
    }
    fn value(&self) -> isize {
        match self {
            Monkey::Value(i) => *i,
            _ => unreachable!(),
        }
    }
    fn dependant(&self) -> (&'static str, &'static str) {
        match self {
            Monkey::Dependant(e1, _, e2) => (e1, e2),
            _ => unreachable!(),
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
                        Monkey::Value(f(monkeys[e1].value(), monkeys[e2].value()))
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
    monkeys["root"].value()
}

#[derive(Debug, Clone)]
enum Expr {
    Value(isize),
    Dependant(Box<Expr>, fn(isize, isize) -> isize, Box<Expr>),
    Unknown,
}
impl Expr {
    fn create_reverse(self, rhs: Box<Expr>) -> (Box<Expr>, Box<Expr>) {
        println!("{self:#?}");
        match self {
            Expr::Dependant(e1, f, e2) => {
                let add: fn(_, isize) -> _ = isize::add;
                let min: fn(_, isize) -> _ = isize::min;
                let mul: fn(_, isize) -> _ = isize::mul;
                let div: fn(_, isize) -> _ = isize::div;
                let f = f as usize;
                let add = add as usize;
                let min = min as usize;
                let mul = mul as usize;
                let div = div as usize;
                //println!("{} {:?}", f, (add, min, mul, div));
                let rev = match f {
                    x if x == add => isize::min,
                    x if x == min => isize::add,
                    x if x == mul => isize::div,
                    x if x == div => isize::mul,
                    _ => unreachable!(),
                };
                (e1, Box::new(Expr::Dependant(rhs, rev, e2)))
            }
            _ => unreachable!(),
        }
    }
}

pub fn part2() -> isize {
    let mut monkeys = input();
    let rhs = {
        let rhs = monkeys["root"].dependant().1;
        let mut stack = vec![rhs];
        while !stack.is_empty() {
            let top = stack[stack.len() - 1];
            match monkeys.get(top).unwrap() {
                Monkey::Value(_) => {
                    stack.pop();
                }
                Monkey::Dependant(e1, f, e2) => {
                    if monkeys[e1].is_value() && monkeys[e2].is_value() {
                        *monkeys.get_mut(top).unwrap() =
                            Monkey::Value(f(monkeys[e1].value(), monkeys[e2].value()))
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
        monkeys[rhs].value()
    };
    let lhs = monkeys["root"].dependant().0;
    println!(
        "{:#?}",
        map_to_tree(lhs, &monkeys).create_reverse(Box::new(Expr::Value(rhs)))
    );

    0
}

fn map_to_tree(node: &str, map: &HashMap<&str, Monkey>) -> Box<Expr> {
    if node == "humn" {
        Box::new(Expr::Unknown)
    } else {
        match map[node] {
            Monkey::Value(i) => Box::new(Expr::Value(i)),
            Monkey::Dependant(e1, f, e2) => Box::new(Expr::Dependant(
                map_to_tree(e1, map),
                f,
                map_to_tree(e2, map),
            )),
        }
    }
}
