use std::ops::{Add, Mul};

struct Monkey {
    items: Vec<u128>,
    operation: (fn(u128, u128) -> u128, u128),
    test: u128,
    actions: [usize; 2],
    count: u128,
}

type Is<'l> = &'l mut dyn Iterator<Item = &'static str>;
fn parse_splat(splat: Is) -> impl Iterator<Item = &'static str> {
    splat
        .next()
        .unwrap()
        .split([' ', ':', ','])
        .filter(|s| !s.is_empty())
}
fn parse_splat_skip(skip: usize, splat: Is) -> impl Iterator<Item = &'static str> {
    parse_splat(splat).skip(skip)
}
fn parse_splat_nth(nth: usize, splat: Is) -> &'static str {
    parse_splat(splat).nth(nth).unwrap()
}

fn parse_input() -> impl Iterator<Item = Monkey> {
    include_str!("../inputs/2022/day11.input")
        .split("\n\n")
        .map(|monkey_block| {
            let mut splat = monkey_block.lines().skip(1);
            Monkey {
                items: parse_splat_skip(2, &mut splat)
                    .map(|s| s.parse::<u128>().unwrap())
                    .collect::<Vec<_>>(),
                operation: {
                    let mut splat = parse_splat_skip(4, &mut splat);
                    let op = match splat.next().unwrap() {
                        "*" => u128::mul,
                        _ => u128::add,
                    };
                    let val = match splat.next().unwrap() {
                        "old" => 0,
                        x => x.parse::<u128>().unwrap(),
                    };
                    (op, val)
                },
                test: parse_splat_nth(3, &mut splat).parse::<u128>().unwrap(),
                actions: [
                    parse_splat_nth(5, &mut splat).parse::<usize>().unwrap(),
                    parse_splat_nth(5, &mut splat).parse::<usize>().unwrap(),
                ],
                count: 0,
            }
        })
}

fn solver<const N: usize>(mut monkeys: Vec<Monkey>, differ: Box<dyn Fn(u128) -> u128>) -> u128 {
    for _ in 0..N {
        for m in 0..monkeys.len() {
            for _ in 0..monkeys[m].items.len() {
                monkeys[m].count += 1;
                let i = monkeys[m].items.pop().unwrap();
                let new = differ(match &monkeys[m].operation {
                    (fun, 0) => fun(i, i),
                    (fun, x) => fun(i, *x),
                });
                let monke = monkeys[m].actions[(new % monkeys[m].test != 0) as usize];
                monkeys[monke].items.push(new)
            }
        }
    }
    monkeys.sort_by_key(|f| f.count);
    monkeys[monkeys.len() - 1].count * monkeys[monkeys.len() - 2].count
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
