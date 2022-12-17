use std::collections::BTreeMap;

use pathfinding::prelude::dijkstra;

#[derive(Debug, Clone)]
struct Valve {
    flow_rate: isize,
    connections: Vec<(Str, isize)>,
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
            let connections = splat.map(|i| (i, 1)).collect::<Vec<_>>();
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
    print!("AA ");
    let mut count = 1;
    let mut flow = 0;
    let mut released = map
        .iter()
        .filter(|(_, v)| v.flow_rate == 0)
        .map(|(k, _)| *k)
        .collect::<Vec<_>>();

    while count < 30 {
        if let Some((target, _, path, _, _)) = map
            .iter()
            .filter(|(k, _)| !released.contains(k))
            .map(|(k, v)| {
                let (path, cost) = dijk(current, k, &map);
                //println!("{k:?} {}", cost as f64 / v.flow_rate as f64);
                let prio = v.flow_rate as f64 / cost as f64;
                //println!("\t{k} {prio}");
                (k, v, path, cost, prio)
            })
            .min_by(|(_, _, _, _, c1), (_, _, _, _, c2)| c2.total_cmp(c1))
        {
            for _p in path {
                count += 1;
                flow += released.iter().map(|s| map[s].flow_rate).sum::<isize>();
                print!("{_p} ");
                if count >= 30 {
                    //break 'cringe;
                }
            }
            //println!("{target} :{count}");
            current = target;
            released.push(current);
        }
        count += 1;
        // öppna DD 2, BB 5, JJ 9, HH 17, EE 21, CC 24
        // AA DD CC BB AA II JJ II AA DD EE FF GG HH GG FF EE DD CC
        // AA DD CC BB AA II JJ II AA DD EE FF GG HH GG FF EE DD CC

        flow += released.iter().map(|s| map[s].flow_rate).sum::<isize>();
    }
    // loop time < 30:
    //  Calculate best next option
    //      cost to go there / preasure it will release
    //  Path find there
    all_paths("AA", &map);
    flow
}

fn dijk(from: Str, target: Str, map: &Map) -> (Vec<Str>, isize) {
    let (mut path, cost) =
        dijkstra(&from, |p| map[p].connections.clone(), |p| p == &target).unwrap();
    path.remove(0);
    (path, cost)
}
fn all_paths(from: Str, map: &Map) {
    pathfinding::directed::dfs::dfs_reach(from, |s| map[s].connections.iter().map(|(u, _)| *u))
        .for_each(|p| println!("{p}"));
}

pub fn part2() -> i32 {
    0
}
