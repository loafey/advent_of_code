use crate::utils::{load_string, IteratorEvalExt};
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

fn pulse_shitter(map: &mut Map) -> (usize, usize) {
    let mut work_stack =
        VecDeque::from([("broadcaster".to_string(), "broadcaster".to_string(), Low)]);
    let (mut lows, mut highs) = (0, 0);
    while !work_stack.is_empty() {
        let (me, sender, pulse) = work_stack.pop_front().unwrap();
        if matches!(&me[..], "dl" | "rv" | "bt" | "fr") && pulse == Low {
            println!("{me} {pulse:?}");
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
    // map.iter().for_each(|(k, v)| println!("{k}: \t {v:?}"));
    // println!();

    let (mut lows, mut highs) = (0, 0);
    for _ in 0..1000 {
        let (nlow, nhigh) = pulse_shitter(&mut map);
        lows += nlow;
        highs += nhigh;
        // map.iter().for_each(|(k, v)| println!("{k}: \t {v:?}"));
        // println!();
    }

    lows * highs
}

pub fn part2() -> usize {
    let mut map = input();

    let rx_holder = map
        .iter()
        .find(|(_, m)| m.connected.contains(&"rx".to_string()))
        .map(|(n, m)| {
            println!("{m:?}");
            n
        })
        .unwrap()
        .clone();

    let mut i = 0;
    loop {
        i += 1;
        pulse_shitter(&mut map);
        if let Conjuction { state } = &map[&rx_holder].mtype {
            // state.iter().for_each(|(s, _)| println!("{:?}", map[s]));
            if state.iter().any(|(_, v)| *v) {
                println!("{i}: {state:?}");
            };
        }
    }
    0
}
