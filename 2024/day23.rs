use std::collections::BTreeSet;

use rustc_hash::{FxHashMap, FxHashSet};

pub fn part1() -> i64 {
    let mut map: FxHashMap<&'static str, FxHashSet<&'static str>> = FxHashMap::default();
    include_str!("../inputs/2024/day23.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.split_once('-').unwrap())
        .for_each(|(a, b)| {
            map.entry(a).or_default().insert(b);
            map.entry(b).or_default().insert(a);
        });
    let mut combos = BTreeSet::new();
    for (comp, cons) in &map {
        for c in cons {
            let n = map.get(c).unwrap();
            for n in n {
                if cons.contains(n) {
                    let map = BTreeSet::from([comp, c, n]);
                    combos.insert(map);
                }
            }
        }
    }
    let mut ans = 0;
    for c in combos {
        for k in c {
            if k.starts_with('t') {
                ans += 1;
                break;
            }
        }
        // println!("{c:?}")
    }
    ans
}
pub fn part2() -> String {
    let mut map: FxHashMap<&'static str, FxHashSet<&'static str>> = FxHashMap::default();
    include_str!("../inputs/2024/day23.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.split_once('-').unwrap())
        .for_each(|(a, b)| {
            map.entry(a).or_default().insert(b);
            map.entry(b).or_default().insert(a);
        });

    // let mut combos = BTreeSet::new();
    fn recurr(
        map: &FxHashMap<&'static str, FxHashSet<&'static str>>,
        cons: BTreeSet<&'static str>,
    ) -> BTreeSet<BTreeSet<&'static str>> {
        let mut res = BTreeSet::new();

        let mut cons_res = cons.clone();
        for c in cons {
            for k in map.get(c).unwrap() {
                let kk = map.get(k).unwrap();
                if cons_res.iter().all(|k| kk.contains(k)) {
                    cons_res.insert(k);
                }
            }
        }
        res.insert(cons_res);

        res
    }
    let mut combos = BTreeSet::new();
    for (c, k) in &map {
        for k in k {
            if map.get(k).map(|v| v.contains(c)).unwrap_or_default() {
                let cons = BTreeSet::from([*c, *k]);
                combos.append(&mut recurr(&map, cons));
            }
        }
    }
    let mut max = BTreeSet::new();
    for c in combos {
        if c.len() > max.len() {
            max = c;
        }
    }

    max.into_iter().collect::<Vec<_>>().join(",")
}
