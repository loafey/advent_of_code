use std::{collections::BTreeMap, fs::File, io::Write};

use rustc_hash::FxHashMap;

pub fn part1() -> usize {
    let (vals, wires) = include_str!("../inputs/2024/day24.input")
        .split_once("\n\n")
        .unwrap();
    let mut vals = vals
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            let (a, b) = s.split_once(": ").unwrap();
            (a, b == "1")
        })
        .collect::<FxHashMap<_, _>>();
    let mut wires = wires
        .lines()
        .filter(|s| !s.is_empty())
        .map(|v| {
            let mut split = v.split_whitespace();
            let a = split.next().unwrap();
            let op = split.next().unwrap();
            let b = split.next().unwrap();
            let t = split.nth(1).unwrap();
            (a, op, b, t)
        })
        .collect::<Vec<_>>();

    let mut z_map = BTreeMap::new();

    let mut i = 0;
    while !wires.is_empty() {
        if i > wires.len() {
            i = 0;
        }
        let (a, op, b, t) = wires[i];
        let Some(a) = vals.get(a).copied() else {
            i += 1;
            continue;
        };
        let Some(b) = vals.get(b).copied() else {
            i += 1;
            continue;
        };
        let res = match op {
            "AND" => a && b,
            "XOR" => a ^ b,
            "OR" => a || b,
            _ => panic!(),
        };
        if t.starts_with('z') {
            z_map.insert(t, res);
        } else {
            vals.insert(t, res);
        }
        wires.remove(i);
        i = 0;
    }

    let mut ans = 0;
    for (i, v) in z_map.into_values().enumerate() {
        if v {
            ans |= 1 << i;
        }
    }

    ans
}
pub fn part2() -> i64 {
    let (vals, wires) = include_str!("../inputs/2024/day24.input")
        .split_once("\n\n")
        .unwrap();

    let og_wires = wires
        .lines()
        .filter(|s| !s.is_empty())
        .map(|v| {
            let mut split = v.split_whitespace();
            let a = split.next().unwrap();
            let op = split.next().unwrap();
            let b = split.next().unwrap();
            let t = split.nth(1).unwrap();
            (a, op, b, t)
        })
        .collect::<Vec<_>>();

    let mut s = "digraph doa {\n".to_string();
    for (a, op, b, t) in &og_wires {
        let middle = format!("{a}_{op}_{b}");
        s += &format!("\t{a} -> {middle} [label=\"\"] \n");
        s += &format!("\t{b} -> {middle} [label=\"\"] \n");
        s += &format!("\t{middle} -> {t} [label=\"\"]\n");
        if a.starts_with('x') {
            s += &format!("\t{a} [style=filled,fillcolor=\"red\"]\n");
        }
        if b.starts_with('x') {
            s += &format!("\t{b} [style=filled,fillcolor=\"red\"]\n");
        }
        if a.starts_with('y') {
            s += &format!("\t{a} [style=filled,fillcolor=\"cyan\"]\n");
        }
        if b.starts_with('y') {
            s += &format!("\t{b} [style=filled,fillcolor=\"cyan\"]\n");
        }
        if t.starts_with('z') {
            s += &format!("\t{t} [style=filled,fillcolor=\"green\"]\n");
        }
        s += &format!("\t{middle} [style=filled,fillcolor=\"pink\",label=\"{op}\"]\n")
    }
    s += "}";

    let mut f = File::create("graph.dot").unwrap();
    f.write_all(s.as_bytes()).unwrap();

    let mut num_test = 0;
    let mut y_test = 0;
    while y_test < 44 {
        if num_test >= 44 {
            num_test = 0;
            y_test += 1;
        }
        let mut wires = og_wires.clone();
        let mut x_map = BTreeMap::new();
        let mut y_map = BTreeMap::new();
        let mut vals = vals
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| {
                let (a, b) = s.split_once(": ").unwrap();
                if a.starts_with('x') {
                    x_map.insert(a, x_map.len() == num_test);
                    (a, x_map.len() - 1 == num_test)
                } else if a.starts_with('y') {
                    y_map.insert(a, y_map.len() == y_test);
                    (a, y_map.len() - 1 == y_test)
                } else {
                    (a, b == "1")
                }
            })
            .collect::<FxHashMap<_, _>>();

        let mut z_map = BTreeMap::new();

        let mut i = 0;
        while !wires.is_empty() {
            if i > wires.len() {
                i = 0;
            }
            let (a_s, op, b_s, t) = wires[i];
            let Some(a) = vals.get(a_s).copied() else {
                i += 1;
                continue;
            };
            let Some(b) = vals.get(b_s).copied() else {
                i += 1;
                continue;
            };
            let res = match op {
                "AND" => a && b,
                "XOR" => a ^ b,
                "OR" => a || b,
                _ => panic!(),
            };
            // if a_s == "x00" {
            // eprintln!("{a_s}({a}) {op} {b_s}({b}) = {res}");
            // }
            if t.starts_with('z') {
                z_map.insert(t, res);
            } else {
                vals.insert(t, res);
            }
            wires.remove(i);
            i = 0;
        }

        let x = to_num(x_map);
        let y = to_num(y_map);
        let _start = format!("{num_test} {y_test}: {x} + {y} = ");
        //eprintln!("{start}{}", x + y);
        let z = to_num(z_map);
        // eprintln!("{}{}", " ".repeat(start.len()), z);
        if x + y != z {
            break;
        }
        num_test += 1;
    }

    // eprintln!(
    //     "{:?}",
    //     BTreeSet::from(["z07", "gmt", "cbj", "qjj", "z18", "dmn", "z35", "cfk"])
    //         .into_iter()
    //         .collect::<Vec<_>>()
    //         .join(",")
    // );
    // eprintln!(
    //     "{:?}",
    //     BTreeSet::from(["aaa", "eee", "ooo", "z99", "bbb", "ccc", "aoc", "z24"])
    //         .into_iter()
    //         .collect::<Vec<_>>()
    //         .join(",")
    // );
    0
}
// cbj,cfk,dmn,gmt,qjj,z07,z18,z35
// cfk,cjj,dmn,mvw,qjj,z07,z18,z35
fn to_num(map: BTreeMap<&str, bool>) -> i64 {
    let mut ans = 0;
    for (i, v) in map.into_values().enumerate() {
        if v {
            ans |= 1 << i;
        }
    }
    ans
}
