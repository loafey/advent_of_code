use std::collections::BTreeMap;

use memoize::memoize;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Valve {
    flow_rate: isize,
    connections: Vec<Str>,
}
type Map = BTreeMap<Str, Valve>;

type Str = &'static str;

pub fn part1() -> isize {
    let map = include_str!("input/day16.input")
        .lines()
        .map(|s| {
            let mut splat = s
                .split(|c| c == '=' || c == ';' || c == ',' || c == ' ')
                .filter(|s| !s.is_empty())
                .skip(1);
            let name = splat.next().unwrap();
            for _ in 0..3 {
                splat.next();
            }
            let flow_rate = splat.next().unwrap().parse::<isize>().unwrap();
            for _ in 0..4 {
                splat.next();
            }
            let connections = splat.collect::<Vec<_>>();
            (
                name,
                Valve {
                    flow_rate,
                    connections,
                },
            )
        })
        .collect::<Map>();

    rinzal_dp(map.clone(), "AA", 29)
}

#[memoize]
fn rinzal_dp(map: Map, current: Str, mins: isize) -> isize {
    match mins {
        0 => 0,
        _ => {
            let current_flow_rate = map[current].flow_rate;
            let open = (mins * current_flow_rate)
                + rinzal_dp(
                    {
                        let mut map = map.clone();
                        map.get_mut(current).unwrap().flow_rate = 0;
                        map
                    },
                    current,
                    mins - 1,
                );
            let mov = map[current]
                .connections
                .iter()
                .map(|x| rinzal_dp(map.clone(), x, mins - 1))
                .max()
                .unwrap();
            mov.max(open)
        }
    }
}

pub fn part2() -> &'static str {
    "I can't do this"
}
