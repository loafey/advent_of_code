use crate::utils::{load_string, IteratorEvalExt};
use std::collections::{HashMap, HashSet};
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

    for (s, m) in map
        .iter()
        .filter(|(_, a)| !matches!(a.mtype, Conjuction { .. }))
        .map(|(a, b)| (a.clone(), b.clone()))
        .eval()
    {
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

fn pulse_shitter(me: &str, sender: &str, pulse: Signal, map: &mut Map) -> (usize, usize) {
    let mut work_stack = vec![(me, sender, pulse)];
    let (mut lows, mut highs) = match pulse {
        Low => (1, 0),
        High => (0, 1),
    };
    // println!("{sender} -{pulse:?}> {me}");
    if let Some(node) = map.get_mut(me) {
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
                        let (nlow, nhigh) = pulse_shitter(&con, me, message, map);
                        lows += nlow;
                        highs += nhigh;
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
                    let (nlow, nhigh) = pulse_shitter(&con, me, message, map);
                    lows += nlow;
                    highs += nhigh;
                }
            }
            Broadcaster => {
                for con in connections {
                    let (nlow, nhigh) = pulse_shitter(&con, me, Low, map);
                    lows += nlow;
                    highs += nhigh;
                }
            }
        }
    } else if me == "output" {
    } else {
        unreachable!()
    }
    (lows, highs)
}

pub fn part1() -> usize {
    let mut map = input();
    map.iter().for_each(|(k, v)| println!("{k}: \t {v:?}"));
    println!();

    let (mut lows, mut highs) = (0, 0);
    for _ in 0..1 {
        let (nlow, nhigh) = pulse_shitter("broadcaster", "button", Low, &mut map);
        lows += nlow;
        highs += nhigh;
        map.iter().for_each(|(k, v)| println!("{k}: \t {v:?}"));
        println!();
    }
    println!("{lows} {highs}");

    lows * highs
}

pub fn part2() -> isize {
    0
}
