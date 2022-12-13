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
        println!("{}: {}", i + 1, battle(data1.clone(), data2.clone()));
    }
    input
        .into_iter()
        .enumerate()
        .filter(|(_, Pair { data1, data2 })| battle(data1.clone(), data2.clone()))
        .map(|(i, _)| i + 1)
        .sum()
}
fn battle(data1: Data, data2: Data) -> bool {
    let mut rights = true;
    match (data1, data2) {
        (Data::List(mut v1), Data::List(mut v2)) if v1.len() == 1 && v1[0].is_value() => {
            let val = v1.pop_front().unwrap();
            while let Some(right) = v2.pop_front() {
                rights = rights && battle(val.clone(), right)
            }
        }
        (Data::List(mut v1), Data::List(mut v2)) if v2.len() == 1 && v2[0].is_value() => {
            let val = v2.pop_front().unwrap();
            while let Some(left) = v1.pop_front() {
                rights = rights && battle(left, val.clone())
            }
        }
        (Data::List(mut v1), Data::List(mut v2)) => {
            //println!("list list: {v1:?} {v2:?}");
            if v1.len() > v2.len() {
                return false;
            }

            while let Some(right) = v2.pop_front() {
                if let Some(left) = v1.pop_front() {
                    let res = battle(left, right);
                    if !res {
                        rights = false;
                        break;
                    }
                    rights = rights && res;
                } else {
                    //rights = rights && true;
                    break;
                }
            }
        }
        (Data::Value(left), Data::Value(right)) => {
            //println!("val val: {left:?} {right:?}");
            if left > right {
                rights = false;
            }
        }
        (Data::List(l), Data::Value(x)) => {
            //println!("list val: {l:?} {x:?}");
            rights = rights && battle(Data::List(l), Data::List(vec![Data::Value(x)].into()));
        }
        (Data::Value(x), Data::List(l)) => {
            //println!("val list: {x:?} {l:?}");
            rights = rights && battle(Data::List(vec![Data::Value(x)].into()), Data::List(l));
        }
    }

    rights
}

pub fn part2() -> i32 {
    0
}
