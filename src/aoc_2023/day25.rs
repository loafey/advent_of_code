use std::collections::{BTreeMap as Map, BTreeSet as Set, VecDeque};

use pathfinding::num_traits::PrimInt;
use rayon::iter::{self, IntoParallelRefIterator, ParallelIterator as _};

use crate::utils::load_string;

fn input() -> Map<String, Set<String>> {
    let mut map = load_string("inputs/2023/day25.input")
        .lines()
        .map(|r| {
            let mut splat = r.split(": ");
            let name = splat.next().unwrap().to_string();
            let rest = splat
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Set<_>>();
            (name, rest)
        })
        .collect::<Map<_, _>>();
    let copy = map.clone();
    // let mut s = "digraph G {".to_string();
    // for (n, u) in map.iter() {
    //     for u in u {
    //         s.push_str(&format!("   {n} -> {u}\n"));
    //     }
    // }
    // s += "}";
    // println!("{s}");
    copy.into_iter().for_each(|(n, v)| {
        for v in v {
            if let Some(ov) = map.get_mut(&v) {
                ov.insert(n.clone());
            } else {
                let ov = Set::from([n.clone()]);
                map.insert(v.clone(), ov);
            }
        }
    });
    map
}
fn remove_connection(map: &mut Map<String, Set<String>>, start: &String, end: &String) {
    let start_m = map.get_mut(start).unwrap();
    start_m.remove(end);
    let end_m = map.get_mut(end).unwrap();
    end_m.remove(start);
}

fn groupoid(map: &Map<String, Set<String>>) -> Vec<Set<String>> {
    let mut map = map.clone();

    let mut res = Vec::new();

    while !map.is_empty() {
        let (n, v) = map.pop_first().unwrap();
        let mut set = Set::from([n.clone()]);
        let mut stack = v;
        while let Some(v) = stack.pop_first() {
            if !set.contains(&v) {
                if let Some(conns) = map.remove(&v) {
                    let conns = conns.clone();
                    conns.into_iter().for_each(|s| {
                        stack.insert(s);
                    });
                    set.insert(v);
                }
            }
        }
        res.push(set);
    }

    res
}

// hfx | pzl
// bvb | cmg
// nvd | jqt

// &"hfx".to_string(), &"pzl".to_string(),
// &"bvb".to_string(), &"cmg".to_string(),
// &"nvd".to_string(), &"jqt".to_string(),
pub fn part1() -> usize {
    let mut map = input();
    let mut cutters = Set::new();
    map.iter().for_each(|(n, u)| {
        for check in u {
            let check_b = &map[check];
            if !u.iter().any(|u| check_b.contains(u)) {
                // Double check if n neighbors is connected to checks neighbors
                let ab = (n.clone(), check.clone());
                let ba = (check.clone(), n.clone());

                if !cutters.contains(&ab) && !cutters.contains(&ba) {
                    cutters.insert(ab);
                }
            }
        }
    });
    // let mut to_be_removed = Vec::new();
    // for (a, b) in &cutters {
    //     for (c, d) in &cutters {
    //         if (a, b) == (c, d) {
    //             continue;
    //         }
    //         if map[a].contains(c) && map[b].contains(d) {
    //             to_be_removed.push((a.clone(), b.clone()));
    //             to_be_removed.push((c.clone(), d.clone()));
    //         }
    //         // println!("Pair pair {:?} {:?}", (a, b), (c, d))
    //     }
    // }
    // for t in to_be_removed {
    //     cutters.remove(&t);
    // }
    cutters.iter().for_each(|(a, b)| println!("{a} - {b}"));
    let (a, b) = cutters.pop_first().unwrap();
    let (c, d) = cutters.pop_first().unwrap();
    let (e, f) = cutters.pop_first().unwrap();

    if !cutters.is_empty() {
        panic!()
    }
    remove_connection(&mut map, &a, &b);
    remove_connection(&mut map, &c, &d);
    remove_connection(&mut map, &e, &f);
    let g = groupoid(&map);
    g[0].len() * g[1].len()
}

pub fn part2() -> usize {
    0
}
