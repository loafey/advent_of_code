use std::ops::{Add, Mul};

#[derive(Debug, Clone)]
enum Value {
    Old,
    Num(u128),
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u128>,
    operation: (fn(u128, u128) -> u128, Value),
    test: u128,
    true_action: usize,
    false_action: usize,
}
impl Clone for Monkey {
    fn clone(&self) -> Self {
        Self {
            items: Vec::new(),
            operation: self.operation.clone(),
            test: self.test,
            true_action: self.true_action,
            false_action: self.false_action,
        }
    }
}

fn parse_input() -> impl Iterator<Item = Monkey> {
    include_str!("input/day11.input")
        .split("\n\n")
        .map(|monkey_block| {
            let mut splat = monkey_block.lines();
            let _monkey_num = splat
                .next()
                .unwrap()
                .split(|c| c == ' ' || c == ':')
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let items = splat
                .next()
                .unwrap()
                .split(|c| c == ' ' || c == ':' || c == ',')
                .filter(|s| !s.is_empty())
                .skip(2)
                .map(|s| s.parse::<u128>().unwrap())
                .collect::<Vec<_>>();
            let operation: (fn(u128, u128) -> u128, _) = {
                let mut splat = splat
                    .next()
                    .unwrap()
                    .split(|c| c == ' ' || c == ':')
                    .filter(|s| !s.is_empty())
                    .skip(4);
                (
                    match splat.next().unwrap() {
                        "*" => u128::mul,
                        "+" => u128::add,
                        _ => {
                            unreachable!()
                        }
                    },
                    match splat.next().unwrap() {
                        "old" => Value::Old,
                        x => Value::Num(x.parse::<u128>().unwrap()),
                    },
                )
            };
            let test = splat
                .next()
                .unwrap()
                .split(|c| c == ' ' || c == ':')
                .filter(|s| !s.is_empty())
                .nth(3)
                .unwrap()
                .parse::<u128>()
                .unwrap();
            let true_action = splat
                .next()
                .unwrap()
                .split(|c| c == ' ' || c == ':')
                .filter(|s| !s.is_empty())
                .nth(5)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let false_action = splat
                .next()
                .unwrap()
                .split(|c| c == ' ' || c == ':')
                .filter(|s| !s.is_empty())
                .nth(5)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            Monkey {
                items,
                operation,
                test,
                true_action,
                false_action,
            }
        })
}

pub fn part1() -> u128 {
    let mut monkeys = parse_input().collect::<Vec<_>>();
    let mut inspect_amounts = vec![0; monkeys.len()];
    for _ in 0..20 {
        //let mut new_monkeys = monkeys.clone();
        for m in 0..monkeys.len() {
            for _ in 0..monkeys[m].items.len() {
                inspect_amounts[m] += 1;
                let new = match monkeys[m].operation {
                    (op, Value::Num(x)) => op(monkeys[m].items[0], x),
                    (op, Value::Old) => op(monkeys[m].items[0], monkeys[m].items[0]),
                } / 3;
                monkeys[m].items.remove(0);
                if new % monkeys[m].test == 0 {
                    let trur = monkeys[m].true_action;
                    monkeys[trur].items.push(new)
                } else {
                    let flur = monkeys[m].false_action;
                    monkeys[flur].items.push(new)
                }
            }
        }
        //monkeys = new_monkeys
    }
    inspect_amounts.sort();
    inspect_amounts[inspect_amounts.len() - 1] * inspect_amounts[inspect_amounts.len() - 2]
}

pub fn part2() -> u128 {
    let mut monkeys = parse_input().collect::<Vec<_>>();
    let mut inspect_amounts = vec![0; monkeys.len()];
    for _ in 0..1000 {
        //let mut new_monkeys = monkeys.clone();
        for m in 0..monkeys.len() {
            for _ in 0..monkeys[m].items.len() {
                inspect_amounts[m] += 1;
                let i = monkeys[m].items.pop().unwrap();
                println!("{i}");
                let new = match monkeys[m].operation {
                    (op, Value::Num(x)) => op(i, x),
                    (op, Value::Old) => op(i, i),
                };
                //monkeys[m].items.remove(0);
                if new % monkeys[m].test == 0 {
                    let trur = monkeys[m].true_action;
                    monkeys[trur].items.push(new)
                } else {
                    let flur = monkeys[m].false_action;
                    monkeys[flur].items.push(new)
                }
            }
        }
        //monkeys = new_monkeys
    }
    println!("{inspect_amounts:?}");
    inspect_amounts.sort();
    inspect_amounts[inspect_amounts.len() - 1] * inspect_amounts[inspect_amounts.len() - 2]
}
