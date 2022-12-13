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
impl Data {}
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
    let mut sum = 0;
    #[allow(clippy::never_loop)]
    for (i, Pair { data1, data2 }) in input.iter().enumerate() {
        //println!("{data1:?}\n{data2:?}");
        let i = i + 1;
        let res = battle(data1.clone(), data2.clone());
        println!("{i}: {res:?}");
        if let Status::Ok = res {
            sum += i
        }
    }
    sum
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

        (Data::List(l), v) => battle(Data::List(l), Data::List(vec![v].into())),
        (v, Data::List(l)) => battle(Data::List(vec![v].into()), Data::List(l)),

        (Data::Value(l), Data::Value(r)) if l < r => Status::Ok,
        (Data::Value(l), Data::Value(r)) if l == r => Status::Continue,
        (Data::Value(_), Data::Value(_)) => Status::Bad,
    }
}

pub fn part2() -> i32 {
    0
}
