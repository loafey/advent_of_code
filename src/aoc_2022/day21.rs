use std::collections::HashMap;

#[derive(Debug)]
enum Monkey {
    Value(isize),
    Dependant(&'static str, Func, &'static str),
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum Func {
    Add,
    Sub,
    Mul,
    Div,
}

impl std::fmt::Debug for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
        }
    }
}
impl Func {
    fn rev(self) -> Self {
        match self {
            Func::Add => Func::Sub,
            Func::Sub => Func::Add,
            Func::Mul => Func::Div,
            Func::Div => Func::Mul,
        }
    }
    fn eval(&self, a: isize, b: isize) -> isize {
        match self {
            Func::Add => a + b,
            Func::Sub => a - b,
            Func::Mul => a * b,
            Func::Div => a / b,
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
                    "+" => Func::Add,
                    "-" => Func::Sub,
                    "*" => Func::Mul,
                    "/" => Func::Div,
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
                        Monkey::Value(f.eval(monkeys[e1].value(), monkeys[e2].value()))
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

#[derive(Clone)]
enum Expr {
    Value(isize),
    Dependant(Box<Expr>, Func, Box<Expr>),
    Unknown,
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(arg0) => write!(f, "{arg0}"),
            Self::Dependant(arg0, arg1, arg2) => write!(f, "({arg0:?} {arg1:?} {arg2:?})"),
            Self::Unknown => write!(f, "X"),
        }
    }
}
impl Expr {
    fn create_reverse(self, rhs: Box<Expr>) -> (Box<Expr>, Box<Expr>) {
        match self {
            Expr::Dependant(e1, f, e2) => {
                if f == Func::Div && e2.contains_unknown() {
                    (e2, Box::new(Expr::Dependant(e1, f.rev(), rhs)))
                } else if f == Func::Sub {
                    if e1.contains_unknown() {
                        (e1, Box::new(Expr::Dependant(rhs, f.rev(), e2)))
                    } else {
                        (
                            Box::new(Expr::Dependant(e2, Func::Mul, Box::new(Expr::Value(-1)))),
                            Box::new(Expr::Dependant(rhs, Func::Sub, e1)),
                        )
                    }
                } else if e1.contains_unknown() {
                    (e1, Box::new(Expr::Dependant(rhs, f.rev(), e2)))
                } else {
                    (e2, Box::new(Expr::Dependant(rhs, f.rev(), e1)))
                }
            }
            _ => unreachable!(),
        }
    }
    fn is_unknown(&self) -> bool {
        matches!(self, Expr::Unknown)
    }
    fn contains_unknown(&self) -> bool {
        match self {
            Expr::Value(_) => false,
            Expr::Dependant(e1, _, e2) => e1.contains_unknown() || e2.contains_unknown(),
            Expr::Unknown => true,
        }
    }
    fn eval(self) -> isize {
        match self {
            Expr::Value(i) => i,
            Expr::Dependant(e1, f, e2) => f.eval(e1.eval(), e2.eval()),
            Expr::Unknown => todo!(),
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
                            Monkey::Value(f.eval(monkeys[e1].value(), monkeys[e2].value()))
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
    let mut rhs = Box::new(Expr::Value(rhs));
    let mut lhs = map_to_tree(monkeys["root"].dependant().0, &monkeys);
    //println!("{lhs:?} = {rhs:?}");
    while !lhs.is_unknown() {
        (lhs, rhs) = lhs.create_reverse(rhs);
        println!("{lhs:?} = {rhs:?}\n")
    }
    rhs.eval()
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

/*
> 3469696969697
? 3469704905529
< 8401064794714
*/
