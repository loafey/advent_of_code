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

fn solver<const N: usize>(mut monkeys: Vec<Monkey>, differ: Box<dyn Fn(u128) -> u128>) -> u128 {
    let mut inspect_amounts = vec![u128::from(0u32); monkeys.len()];
    for _ in 0..N {
        for m in 0..monkeys.len() {
            for _ in 0..monkeys[m].items.len() {
                inspect_amounts[m] += u128::from(1u32);
                let new = differ(match &monkeys[m].operation {
                    (fun, Value::Num(x)) => fun(monkeys[m].items[0], *x),
                    (fun, Value::Old) => fun(monkeys[m].items[0], monkeys[m].items[0]),
                });
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
    }
    inspect_amounts.sort();
    inspect_amounts[inspect_amounts.len() - 1] * inspect_amounts[inspect_amounts.len() - 2]
}

pub fn part1() -> u128 {
    let monkeys = parse_input().collect::<Vec<_>>();
    solver::<20>(monkeys, Box::new(|a| a / 3))
}

pub fn part2() -> u128 {
    let monkeys = parse_input().collect::<Vec<_>>();
    let diff = monkeys
        .iter()
        .map(|m| m.test)
        .reduce(|a, b| a * b)
        .unwrap_or_default();
    solver::<10000>(monkeys, Box::new(move |a| a % diff))
}
