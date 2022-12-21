use std::collections::HashMap;

#[derive(Debug)]
enum Monkey {
    Value(isize),
    Dependant(&'static str, Func, &'static str),
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
    Expr::from_map(false, "root", &input()).eval()
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

    fn from_map(filter_humn: bool, node: &str, map: &HashMap<&str, Monkey>) -> Box<Expr> {
        if filter_humn && node == "humn" {
            Box::new(Expr::Unknown)
        } else {
            match map[node] {
                Monkey::Value(i) => Box::new(Expr::Value(i)),
                Monkey::Dependant(e1, f, e2) => Box::new(Expr::Dependant(
                    Expr::from_map(filter_humn, e1, map),
                    f,
                    Expr::from_map(filter_humn, e2, map),
                )),
            }
        }
    }
}

pub fn part2() -> isize {
    let monkeys = input();
    let Monkey::Dependant(lhs,_, rhs) = monkeys["root"] else {unreachable!()};
    let mut rhs = Expr::from_map(false, rhs, &monkeys);
    let mut lhs = Expr::from_map(true, lhs, &monkeys);
    while !lhs.is_unknown() {
        (lhs, rhs) = lhs.create_reverse(rhs);
    }
    rhs.eval()
}
