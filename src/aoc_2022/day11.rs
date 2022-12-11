use rug::Integer;

#[derive(Debug, Clone)]
enum Value {
    Old,
    Num(Integer),
}

fn cringe_mul(a: Integer, b: Integer) -> Integer {
    a * b
}
fn cringe_add(a: Integer, b: Integer) -> Integer {
    a + b
}

struct Monkey {
    items: Vec<Integer>,
    operation: (fn(Integer, Integer) -> Integer, Value),
    test: Integer,
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
                .map(|s| s.parse::<Integer>().unwrap())
                .collect::<Vec<_>>();
            let operation: (fn(Integer, Integer) -> Integer, _) = {
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
                        x => Value::Num(x.parse::<Integer>().unwrap()),
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
                .parse::<Integer>()
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

pub fn part1() -> Integer {
    let mut monkeys = parse_input().collect::<Vec<_>>();
    let mut inspect_amounts = vec![Integer::from(0u32); monkeys.len()];
    for _ in 0..20 {
        //let mut new_monkeys = monkeys.clone();
        for m in 0..monkeys.len() {
            for _ in 0..monkeys[m].items.len() {
                inspect_amounts[m] += Integer::from(1u32);
                let new = match &monkeys[m].operation {
                    (fun, Value::Num(x)) => fun(monkeys[m].items[0].clone(), x.clone()),
                    (fun, Value::Old) => {
                        fun(monkeys[m].items[0].clone(), monkeys[m].items[0].clone())
                    }
                } / Integer::from(3u32);
                monkeys[m].items.remove(0);
                if new.clone() % monkeys[m].test.clone() == 0u32 {
                    let trur = monkeys[m].true_action;
                    monkeys[trur].items.push(new.clone())
                } else {
                    let flur = monkeys[m].false_action;
                    monkeys[flur].items.push(new.clone())
                }
            }
        }
        //monkeys = new_monkeys
    }
    println!("{inspect_amounts:?}");
    inspect_amounts.sort();
    inspect_amounts[inspect_amounts.len() - 1].clone()
        * inspect_amounts[inspect_amounts.len() - 2].clone()
}

pub fn part2() -> Integer {
    let mut monkeys = parse_input().collect::<Vec<_>>();
    let mut inspect_amounts = vec![Integer::from(0u32); monkeys.len()];
    for c in 0..10000 {
        println!("{c}");
        //let mut new_monkeys = monkeys.clone();
        for m in 0..monkeys.len() {
            for _ in 0..monkeys[m].items.len() {
                inspect_amounts[m] += Integer::from(1u32);
                let i = monkeys[m].items.pop().unwrap();
                let new = match &monkeys[m].operation {
                    //(Op::Mul, _, Value::Num(x)) if i > monkeys[m].test => (i * x) % monkeys[m].test,
                    (fun, Value::Num(x)) => fun(i.clone(), x.clone()),
                    //(Op::Mul, _, Value::Old) if i > monkeys[m].test => i % monkeys[m].test + 1,
                    (fun, Value::Old) => fun(i.clone(), i.clone()),
                };
                //monkeys[m].items.remove(0);
                if new.clone() % monkeys[m].test.clone() == 0u32 {
                    let trur = monkeys[m].true_action;
                    monkeys[trur].items.push(new.clone())
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
    inspect_amounts[inspect_amounts.len() - 1].clone()
        * inspect_amounts[inspect_amounts.len() - 2].clone()
}
