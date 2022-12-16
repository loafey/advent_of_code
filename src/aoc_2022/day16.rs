use std::collections::BTreeMap;

#[derive(Debug, Clone)]
struct Valve {
    flow_rate: isize,
    connections: Vec<Str>,
}
type Map = BTreeMap<Str, Valve>;
type Str = &'static str;

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
        .collect::<Map>();
    let mut current = "AA";
    let mut released = map
        .iter()
        .filter(|(_, v)| v.flow_rate == 0)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    #[allow(clippy::never_loop)]
    loop {
        map.iter()
            .filter(|(k, _)| !released.contains(k))
            .for_each(|(k, v)| {
                println!("{k:?} {}", v.flow_rate);
            });
        println!("{:?}", dfs("AA", "DD", &map));
        break;
    }
    // loop time < 30:
    //  Calculate best next option
    //      cost to go there / preasure it will release
    //  Path find there
    0
}

fn dfs(from: Str, target: Str, map: &Map) {
    fn go(from: Str, target: Str, map: &Map, visited: &mut Vec<&str>) {
        if from == target {
        } else {
            visited.push(from);
            for n in &map[from].connections {
                if !visited.contains(n) {
                    go(n, target, map, visited)
                }
            }
        }
    }
    go(from, target, map, &mut Vec::new())
}

pub fn part2() -> i32 {
    0
}
