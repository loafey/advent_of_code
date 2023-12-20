use crate::utils::{load_string, IteratorEvalExt, NumExt};
use std::{
    clone,
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::Write,
};
use Signal::*;
use Type::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Signal {
    Low,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Type {
    FlipFlop { state: bool },
    Conjuction { state: HashMap<String, bool> },
    Broadcaster,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Module {
    mtype: Type,
    connected: Vec<String>,
}
type Map = HashMap<String, Module>;

fn input() -> Map {
    let mut map = load_string("inputs/2023/day20.input")
        .lines()
        .map(|s| {
            let (name, list) = s.split_once(" -> ").unwrap();
            let connected = list.split(", ").map(|s| s.to_string()).collect::<Vec<_>>();
            let (name, mtype) = match &name[..1] {
                "%" => (name[1..].to_string(), FlipFlop { state: false }),
                "&" => (
                    name[1..].to_string(),
                    Conjuction {
                        state: HashMap::new(),
                    },
                ),
                _ => (name.to_string(), Broadcaster),
            };
            (name, Module { mtype, connected })
        })
        .collect::<HashMap<_, _>>();

    let conjucations = map
        .iter()
        .filter(|(_, a)| matches!(a.mtype, Conjuction { .. }))
        .map(|(s, _)| s.clone())
        .collect::<HashSet<_>>();

    for (s, m) in map.iter().map(|(a, b)| (a.clone(), b.clone())).eval() {
        for con in &m.connected {
            if conjucations.contains(con) {
                if let Conjuction { state } = &mut map.get_mut(con).unwrap().mtype {
                    state.insert(s.clone(), false);
                }
            }
        }
    }

    map
}

fn pulse_shitter(
    count: usize,
    map: &mut Map,
    which: &[String],
    counter: &mut HashMap<String, usize>,
) -> (usize, usize) {
    let mut work_stack =
        VecDeque::from([("broadcaster".to_string(), "broadcaster".to_string(), Low)]);
    let (mut lows, mut highs) = (0, 0);
    while !work_stack.is_empty() {
        let (me, sender, pulse) = work_stack.pop_front().unwrap();
        if which.contains(&me) && pulse == Low && !counter.contains_key(&me.to_string()) {
            counter.insert(me.to_string(), count);
        }
        // println!("{sender} -{pulse:?}> {me}");
        match pulse {
            Low => lows += 1,
            High => highs += 1,
        }
        // println!("{sender} -{pulse:?}> {me}");
        if let Some(node) = map.get_mut(&me) {
            let connections = node.connected.clone();
            match &mut node.mtype {
                FlipFlop { state } => {
                    if matches!(pulse, Low) {
                        *state = !*state;
                        let message = match *state {
                            true => High,
                            false => Low,
                        };
                        for con in connections {
                            work_stack.push_back((con, me.clone(), message));
                        }
                    }
                }
                Conjuction { state } => {
                    state.insert(
                        sender.to_string(),
                        match pulse {
                            Low => false,
                            High => true,
                        },
                    );
                    let message = if state.values().all(|a| *a) {
                        Low
                    } else {
                        High
                    };
                    for con in connections {
                        work_stack.push_back((con, me.clone(), message));
                    }
                }
                Broadcaster => {
                    for con in connections {
                        work_stack.push_back((con, me.clone(), Low));
                    }
                }
            }
        }
    }
    (lows, highs)
}

pub fn part1() -> usize {
    let mut map = input();

    let (mut lows, mut highs) = (0, 0);
    for _ in 0..1000 {
        let (nlow, nhigh) = pulse_shitter(0, &mut map, &[], &mut HashMap::new());
        lows += nlow;
        highs += nhigh;
    }

    lows * highs
}

pub fn part2() -> usize {
    let mut map = input();

    let rx_holder = map
        .iter()
        .find(|(_, m)| m.connected.contains(&"rx".to_string()))
        .map(|(n, _)| n)
        .unwrap();
    let rs_holder = map
        .iter()
        .filter(|(_, m)| m.connected.contains(rx_holder))
        .map(|(n, _)| n.clone())
        .collect::<Vec<_>>();

    let mut i = 0;
    let mut counter = HashMap::new();
    loop {
        i += 1;
        pulse_shitter(i, &mut map, &rs_holder, &mut counter);
        if counter.len() == rs_holder.len() {
            break;
        }
    }
    counter.into_values().reduce(|a, b| a.lcm(&b)).unwrap()
}
