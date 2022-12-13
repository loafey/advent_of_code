use std::collections::VecDeque;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[serde(untagged)]
enum Data {
    List(VecDeque<Data>),
    Value(usize),
}
impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::List(arg0) => f.debug_list().entries(arg0).finish(),
            Self::Value(arg0) => write!(f, "{arg0}"),
        }
    }
}
impl Data {
    fn is_value(&self) -> bool {
        match self {
            Data::List(_) => false,
            Data::Value(_) => true,
        }
    }
    fn is_less(&self, x: &Data) -> bool {
        match (self, x) {
            (Data::Value(x), Data::Value(y)) => x < y,
            (Data::List(l1), Data::List(l2)) => {
                battle(Data::List(l1.clone()), Data::List(l2.clone()), 0)
            }
            _ => false,
        }
    }
    fn is_same(&self, x: &Data) -> bool {
        match (self, x) {
            (Data::Value(x), Data::Value(y)) => x == y,
            (Data::List(l1), Data::List(l2)) => {
                battle(Data::List(l1.clone()), Data::List(l2.clone()), 0)
            }
            _ => false,
        }
    }
}
#[derive(Debug, Deserialize)]
struct Pair {
    data1: Data,
    data2: Data,
}

pub fn part1() -> usize {
    let input = include_str!("input/day13.input")
        .split("\n\n")
        .map(|pair| {
            let mut lines = pair.lines();
            let d1 = lines.next().unwrap();
            let d2 = lines.next().unwrap();
            let str = format!("{{\"data1\": {d1}, \"data2\": {d2}}}");
            serde_json::from_str::<Pair>(&str).unwrap()
        })
        .collect::<Vec<_>>();
    #[allow(clippy::never_loop)]
    for (i, Pair { data1, data2 }) in input.iter().enumerate() {
        println!("{}: {}", i + 1, battle(data1.clone(), data2.clone(), 0));
        println!()
    }
    input
        .into_iter()
        .enumerate()
        .filter(|(_, Pair { data1, data2 })| battle(data1.clone(), data2.clone(), 0))
        .map(|(i, _)| i + 1)
        .sum()
    //0
}
fn battle(data1: Data, data2: Data, indent: usize) -> bool {
    let i = "\t".repeat(indent + 1);
    match (data1, data2) {
        (Data::List(mut v1), Data::List(mut v2)) if v1.len() == 1 && v1[0].is_value() => {
            println!("{i}list[int] list: {v1:?} {v2:?}");
            let val = v1.pop_front().unwrap();
            while let Some(right) = v2.pop_front() {
                println!("{i}int int: {val:?} {right:?}");
                if val.is_less(&right) {
                    return true;
                } else if val.is_same(&right) {
                    continue;
                } else {
                    return false;
                }
            }
            true
        }
        (Data::List(mut v1), Data::List(mut v2)) if v2.len() == 1 && v2[0].is_value() => {
            println!("{i}list list[int]: {v1:?} {v2:?}");
            let val = v2.pop_front().unwrap();
            while let Some(left) = v1.pop_front() {
                println!("{i}int int: {left:?} {val:?}");
                if left.is_less(&val) {
                    return true;
                } else if left.is_same(&val) {
                    continue;
                } else {
                    return false;
                }
            }
            true
        }
        (Data::List(mut v1), Data::List(mut v2)) => {
            println!("{i}list list: {v1:?} {v2:?}");
            while let Some(right) = v2.pop_front() {
                if let Some(left) = v1.pop_front() {
                    println!("{i}int int: {left:?} {right:?}");
                    if left.is_less(&right) {
                        return true;
                    } else if left.is_same(&right) {
                        continue;
                    } else {
                        return false;
                    }
                } else {
                    return true;
                }
            }
            !v2.is_empty() || v1.is_empty()
        }

        (Data::List(l), Data::Value(x)) => {
            println!("{i}list val: {l:?} {x:?}");
            battle(
                Data::List(l),
                Data::List(vec![Data::Value(x)].into()),
                indent + 1,
            )
        }
        (Data::Value(x), Data::List(l)) => {
            println!("{i}val list: {x:?} {l:?}");
            battle(
                Data::List(vec![Data::Value(x)].into()),
                Data::List(l),
                indent + 1,
            )
        }
        _ => unreachable!(), //(Data::Value(left), Data::Value(right)) => {
                             //    println!("{i}val val: {left:?} {right:?}");
                             //    left < right
                             //}
    }
}

pub fn part2() -> i32 {
    0
}
