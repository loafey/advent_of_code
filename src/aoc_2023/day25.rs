use rand::seq::SliceRandom;

use crate::utils::load_string;
use std::collections::{BTreeSet as Set, HashMap as Map};

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
    // let mut s = "digraph G {\n".to_string();
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
// fn remove_connection(map: &mut Map<String, Set<String>>, start: &String, end: &String) {
//     let start_m = map.get_mut(start).unwrap();
//     start_m.remove(end);
//     let end_m = map.get_mut(end).unwrap();
//     end_m.remove(start);
// }

// fn groupoid(map: &Map<String, Set<String>>) -> Vec<Set<String>> {
//     let mut map = map.into_iter();

//     let mut res = Vec::new();

//     while !map.is_empty() {
//         let (n, v) = map.pop_first().unwrap();
//         let mut set = Set::from([n.clone()]);
//         let mut stack = v;
//         while let Some(v) = stack.pop_first() {
//             if !set.contains(&v) {
//                 if let Some(conns) = map.remove(&v) {
//                     let conns = conns.clone();
//                     conns.into_iter().for_each(|s| {
//                         stack.insert(s);
//                     });
//                     set.insert(v);
//                 }
//             }
//         }
//         res.push(set);
//     }

//     res
// }
pub fn part1() -> usize {
    let mut sizes = Map::new();
    (0..10000)
        .map(|_| {
            let mut rng = rand::rngs::ThreadRng::default();
            let mut all = input().into_iter().collect::<Vec<_>>();
            all.shuffle(&mut rng);
            let start = all.pop().unwrap();
            let mut all = all.into_iter().collect::<Map<_, _>>();
            let mut b = Map::from([start.clone()]);
            for n in start.1 {
                let ns = all.remove(&n).unwrap();
                b.insert(n, ns);
            }
            loop {
                let mut modified = false;
                let mut new_all = Map::new();
                let mut vec = all.into_iter().collect::<Vec<_>>();
                vec.shuffle(&mut rng);
                for (n, u) in vec {
                    if u.iter().filter(|a| b.contains_key(*a)).count() > 1 {
                        modified = true;
                        b.insert(n, u);
                    } else {
                        new_all.insert(n, u);
                    }
                }
                all = new_all;

                if !modified {
                    break;
                }
            }
            all.len() * b.len()
        })
        .filter(|a| *a != 0 && *a > 367290)
        .for_each(|a| {
            if let Some(val) = sizes.get_mut(&a) {
                *val += 1;
            } else {
                sizes.insert(a, 1);
            }
        });
    println!("{sizes:?}");
    0
}

pub fn part2() -> usize {
    0
}
