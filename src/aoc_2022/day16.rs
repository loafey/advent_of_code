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
            let connections = splat.map(|s| (s, 1)).collect::<Vec<_>>();
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
                let (path, cost) = pathfind(current, k, &map);
                //println!("{k:?} {}", cost as f64 / v.flow_rate as f64);
                let prio = (cost as f64).powi(2) / (v.flow_rate) as f64;
                //println!("\t{k} {prio}");
                (k, v, path, cost, prio)
            })
            .min_by(|(_, _, _, _, c1), (_, _, _, _, c2)| c1.total_cmp(c2))
        {
            for _p in path {
                count += 1;
                flow += released.iter().map(|s| map[s].flow_rate).sum::<isize>();
                print!("{_p} ");
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
    flow
}

fn pathfind(from: Str, target: Str, map: &Map) -> (Vec<Str>, isize) {
    //fn go(from: Str, target: Str, map: &Map, visited: &mut Vec<&str>, done: &mut bool) -> Vec<Str> {
    //    let mut buf = vec![from];
    //
    //    for n in &map[from].connections {
    //        if from == target {
    //            *done = true;
    //            visited.push(n);
    //            break;
    //        } else if !visited.contains(n) && !*done {
    //            visited.push(n);
    //            buf.append(&mut go(n, target, map, visited, done));
    //        }
    //    }
    //    buf
    //}
    //go(from, target, map, &mut Vec::new(), &mut false)
    let (mut path, cost) =
        pathfinding::dijkstra(&from, |p| map[p].connections.clone(), |p| p == &target).unwrap();
    path.remove(0);
    (path, cost)
}

pub fn part2() -> i32 {
    0
}
