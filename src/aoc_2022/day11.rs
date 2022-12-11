use num::integer::lcm;

#[derive(Debug, Clone)]
enum Value {
    Old,
    Num(u128),
}

fn cringe_mul(a: u128, b: u128) -> u128 {
    a * b
}
fn cringe_add(a: u128, b: u128) -> u128 {
    a + b
}

struct Monkey {
    items: Vec<u128>,
    operation: (fn(u128, u128) -> u128, Value),
    test: u128,
    true_action: usize,
    false_action: usize,
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
                        "*" => cringe_mul,
                        "+" => cringe_add,
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
    let mut inspect_amounts = vec![u128::from(0u32); monkeys.len()];
    for _ in 0..20 {
        //let mut new_monkeys = monkeys.clone();
        for m in 0..monkeys.len() {
            for _ in 0..monkeys[m].items.len() {
                inspect_amounts[m] += u128::from(1u32);
                let new = match &monkeys[m].operation {
                    (fun, Value::Num(x)) => fun(monkeys[m].items[0], *x),
                    (fun, Value::Old) => fun(monkeys[m].items[0], monkeys[m].items[0]),
                } / u128::from(3u32);
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
    println!("{inspect_amounts:?}");
    inspect_amounts.sort();
    inspect_amounts[inspect_amounts.len() - 1] * inspect_amounts[inspect_amounts.len() - 2]
}

pub fn part2() -> u128 {
    let mut monkeys = parse_input().collect::<Vec<_>>();
    let mut inspect_amounts = vec![u128::from(0u32); monkeys.len()];
    for _ in 0..10000 {
        for m in 0..monkeys.len() {
            for _ in 0..monkeys[m].items.len() {
                inspect_amounts[m] += u128::from(1u32);
                let i = monkeys[m].items.pop().unwrap();
                let new = match &monkeys[m].operation {
                    //(Op::Mul, _, Value::Num(x)) if i > monkeys[m].test => (i * x) % monkeys[m].test,
                    (fun, Value::Num(x)) => fun(i, *x),
                    //(Op::Mul, _, Value::Old) if i > monkeys[m].test => i % monkeys[m].test + 1,
                    (fun, Value::Old) => fun(i, i),
                } % (2 * 17 * 19 * 3 * 5 * 13 * 7 * 11);
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
