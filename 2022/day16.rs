use std::{collections::BTreeMap, ops::Add};

use memoize::memoize;

use utils::load_string;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Valve {
    flow_rate: isize,
    connections: Vec<Str>,
}
type Map = BTreeMap<Str, Valve>;
type Str = (char, char);

fn input() -> Map {
    load_string("inputs/2022/day16.input")
        .lines()
        .map(|s| {
            let mut splat = s
                .split(['=', ';', ',', ' '])
                .filter(|s| !s.is_empty())
                .skip(1);
            let name = {
                let mut s = splat.next().unwrap().chars();
                (s.next().unwrap(), s.next().unwrap())
            };
            for _ in 0..3 {
                splat.next();
            }
            let flow_rate = splat.next().unwrap().parse::<isize>().unwrap();
            for _ in 0..4 {
                splat.next();
            }
            let connections = splat
                .map(|s| {
                    let mut s = s.chars();
                    (s.next().unwrap(), s.next().unwrap())
                })
                .collect::<Vec<_>>();
            (
                name,
                Valve {
                    flow_rate,
                    connections,
                },
            )
        })
        .collect()
}

pub fn part1() -> isize {
    rinzal_dp(
        State {
            flow: 0,
            map: input(),
        },
        ('A', 'A'),
        29,
    )
    .flow
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
    flow: isize,
    map: Map,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State")
            .field("flow", &self.flow)
            .field("map", &self.map)
            .finish()
    }
}
impl Add<isize> for State {
    type Output = State;

    fn add(mut self, rhs: isize) -> Self::Output {
        self.flow += rhs;
        self
    }
}
#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.flow.partial_cmp(&other.flow)
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.flow.partial_cmp(&other.flow).unwrap()
    }
}

#[memoize]
fn rinzal_dp(state: State, a: Str, mins: isize) -> State {
    match mins {
        0 => state,
        _ => {
            let a_open = rinzal_dp(
                {
                    let mut map = state.clone();
                    map.map.get_mut(&a).unwrap().flow_rate = 0;
                    map
                },
                a,
                mins - 1,
            ) + (mins * state.map[&a].flow_rate);

            let a_move = state.map[&a]
                .connections
                .iter()
                .map(|x| rinzal_dp(state.clone(), *x, mins - 1))
                .max()
                .unwrap();

            a_move.max(a_open)
        }
    }
}
pub fn part2() -> isize {
    rinzal_dp(
        rinzal_dp(
            State {
                map: input(),
                flow: 0,
            },
            ('A', 'A'),
            25,
        ),
        ('A', 'A'),
        25,
    )
    .flow
}
