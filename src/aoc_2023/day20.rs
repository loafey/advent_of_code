use crate::utils::{load_string, IteratorEvalExt, NumExt};
use std::collections::{HashMap, HashSet, VecDeque};
use Type::*;

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
    which: &HashSet<String>,
    counter: &mut HashMap<String, usize>,
) -> (usize, usize) {
    let mut work_stack =
        VecDeque::from([("broadcaster".to_string(), "broadcaster".to_string(), false)]);
    let (mut lows, mut highs) = (0, 0);
    while !work_stack.is_empty() {
        let (me, sender, pulse) = work_stack.pop_front().unwrap();
        if which.contains(&me) && !pulse && !counter.contains_key(&me.to_string()) {
            counter.insert(me.to_string(), count);
        }
        match pulse {
            false => lows += 1,
            true => highs += 1,
        }
        if let Some(node) = map.get_mut(&me) {
            let connections = node.connected.clone();
            match &mut node.mtype {
                FlipFlop { state } => {
                    if !pulse {
                        *state = !*state;
                        let message = *state;
                        for con in connections {
                            work_stack.push_back((con, me.clone(), message));
                        }
                    }
                }
                Conjuction { state } => {
                    state.insert(sender.to_string(), pulse);
                    let message = !state.values().all(|a| *a);
                    for con in connections {
                        work_stack.push_back((con, me.clone(), message));
                    }
                }
                Broadcaster => {
                    for con in connections {
                        work_stack.push_back((con, me.clone(), false));
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
        let (nlow, nhigh) = pulse_shitter(0, &mut map, &HashSet::new(), &mut HashMap::new());
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
        .collect::<HashSet<_>>();

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
