use std::collections::BTreeMap;

#[derive(Debug, Clone)]
struct Valve {
    flow_rate: isize,
    connections: Vec<&'static str>,
}

pub fn part1() -> i32 {
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
        .collect::<BTreeMap<_, _>>();
    println!("{map:#?}");
    0
}

pub fn part2() -> i32 {
    0
}
