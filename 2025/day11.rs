use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

fn input() -> HashMap<&'static str, Vec<&'static str>> {
    include_str!("../inputs/2025/day11.input")
        .lines()
        .map(|s| {
            let (node, conns) = s.split_once(':').unwrap();
            (node, conns.split_whitespace().collect::<Vec<_>>())
        })
        .collect()
}

fn explore(
    map: &HashMap<&'static str, Vec<&'static str>>,
    pos: &'static str,
    path: usize,
    filter: bool,
    cache: &mut HashMap<u64, usize>,
) -> usize {
    let mut hasher = DefaultHasher::new();
    pos.hash(&mut hasher);
    path.hash(&mut hasher);
    let hash = hasher.finish();
    if let Some(cache) = cache.get(&hash) {
        return *cache;
    }

    fn inner(
        map: &HashMap<&'static str, Vec<&'static str>>,
        pos: &'static str,
        count: usize,
        filter: bool,
        cache: &mut HashMap<u64, usize>,
    ) -> usize {
        if pos == "out" {
            return (!filter || count == 2) as usize;
        }

        map[pos]
            .iter()
            .map(|v| {
                explore(
                    map,
                    v,
                    count + (matches!(*v, "dac" | "fft")) as usize,
                    filter,
                    cache,
                )
            })
            .sum()
    }

    let res = inner(map, pos, path, filter, cache);
    cache.insert(hash, res);
    res
}

pub fn part1() -> usize {
    explore(&input(), "you", 0, false, &mut HashMap::new())
}

pub fn part2() -> usize {
    explore(&input(), "svr", 0, true, &mut HashMap::new())
}
