use crate::utils::load_string;
use std::collections::VecDeque;

use serde::Deserialize;

#[derive(Deserialize, Clone, PartialEq, Eq)]
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
impl<const N: usize> From<[Data; N]> for Data {
    fn from(value: [Data; N]) -> Self {
        Self::List(value.into())
    }
}
#[derive(Debug, Deserialize)]
struct Pair {
    data1: Data,
    data2: Data,
}

pub fn part1() -> usize {
    let input = load_string("inputs/2022/day13.input")
        .split("\n\n")
        .map(|pair| {
            let mut lines = pair.lines();
            let d1 = lines.next().unwrap();
            let d2 = lines.next().unwrap();
            let str = format!("{{\"data1\": {d1}, \"data2\": {d2}}}");
            serde_json::from_str::<Pair>(&str).unwrap()
        })
        .collect::<Vec<_>>();
    input
        .iter()
        .enumerate()
        .filter_map(|(i, Pair { data1, data2 })| {
            let i = i + 1;
            let res = battle(data1.clone(), data2.clone());
            match res {
                Status::Ok => Some(i),
                _ => None,
            }
        })
        .sum()
}
#[derive(Debug, PartialEq, Eq)]
enum Status {
    Ok,
    Continue,
    Bad,
}
fn battle(data1: Data, data2: Data) -> Status {
    match (data1, data2) {
        (Data::List(l), Data::List(r)) if l.is_empty() && r.is_empty() => Status::Continue,
        (Data::List(mut l), Data::List(mut r)) => {
            while !r.is_empty() && !l.is_empty() {
                let left = l.pop_front().unwrap();
                let right = r.pop_front().unwrap();
                let res = battle(left, right);

                if res != Status::Continue {
                    return res;
                }
            }
            if !l.is_empty() {
                Status::Bad
            } else if l.is_empty() && r.is_empty() {
                Status::Continue
            } else {
                Status::Ok
            }
        }
        (Data::List(l), v) => battle(Data::List(l), Data::from([v])),
        (v, Data::List(l)) => battle(Data::from([v]), Data::List(l)),
        (Data::Value(l), Data::Value(r)) if l < r => Status::Ok,
        (Data::Value(l), Data::Value(r)) if l == r => Status::Continue,
        (Data::Value(_), Data::Value(_)) => Status::Bad,
    }
}

pub fn part2() -> usize {
    let mut input = load_string("inputs/2022/day13.input")
        .split("\n\n")
        .map(|pair| {
            let mut lines = pair.lines();
            let data1 = serde_json::from_str::<Data>(lines.next().unwrap()).unwrap();
            let data2 = serde_json::from_str::<Data>(lines.next().unwrap()).unwrap();
            Pair { data1, data2 }
        })
        .flat_map(|Pair { data1, data2 }| [data1, data2])
        .collect::<Vec<_>>();

    let d2 = Data::from([Data::from([Data::Value(2)])]);
    let d6 = Data::from([Data::from([Data::Value(6)])]);
    input.append(&mut vec![d2.clone(), d6.clone()]);

    input.sort_by(|d1, d2| match battle(d1.clone(), d2.clone()) {
        Status::Bad => std::cmp::Ordering::Greater,
        _ => std::cmp::Ordering::Less,
    });

    input
        .into_iter()
        .enumerate()
        .map(|(i, d)| (i + 1, d))
        .filter(|(_, d)| d == &d2 || d == &d6)
        .map(|(i, _)| i)
        .product()
}
