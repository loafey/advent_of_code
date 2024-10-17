use crate::utils::load_string;
use std::collections::{HashMap, HashSet};

use crate::utils::parse_next;

pub fn part1() -> usize {
    let mut map = HashMap::new();
    load_string("inputs/2018/day3.input").lines().for_each(|s| {
        let mut split = s.split_whitespace().skip(2);
        let coord = split
            .next()
            .map(|s| {
                let mut split = s.split([':', ',']);
                (
                    parse_next::<usize>(&mut split),
                    parse_next::<usize>(&mut split),
                )
            })
            .unwrap();
        let size = split
            .next()
            .map(|s| {
                let mut split = s.split('x');
                (
                    parse_next::<usize>(&mut split),
                    parse_next::<usize>(&mut split),
                )
            })
            .unwrap();

        for x in coord.0..coord.0 + size.0 {
            for y in coord.1..coord.1 + size.1 {
                if let Some(c) = map.get_mut(&(x, y)) {
                    *c += 1;
                } else {
                    map.insert((x, y), 1);
                }
            }
        }
    });
    map.into_iter().filter(|(_, v)| v > &1).count()
}

pub fn part2() -> usize {
    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Fabric {
        id: usize,
        coord: (usize, usize),
        size: (usize, usize),
    }
    impl Fabric {
        fn collides(&self, other: &Self) -> bool {
            self.coord.0 < other.coord.0 + other.size.0
                && self.coord.0 + self.size.0 > other.coord.0
                && self.coord.1 < other.coord.1 + other.size.1
                && self.size.1 + self.coord.1 > other.coord.1
        }
    }

    let map = load_string("inputs/2018/day3.input")
        .lines()
        .map(|s| {
            let mut split = s.split_whitespace();
            let id = split
                .next()
                .and_then(|s| s[1..].parse::<usize>().ok())
                .unwrap();
            split.next();
            let coord = split
                .next()
                .map(|s| {
                    let mut split = s.split([':', ',']);
                    (
                        split.next().and_then(|s| s.parse::<usize>().ok()).unwrap(),
                        split.next().and_then(|s| s.parse::<usize>().ok()).unwrap(),
                    )
                })
                .unwrap();
            let size = split
                .next()
                .map(|s| {
                    let mut split = s.split('x');
                    (
                        parse_next::<usize>(&mut split),
                        parse_next::<usize>(&mut split),
                    )
                })
                .unwrap();
            Fabric { id, coord, size }
        })
        .collect::<Vec<_>>();

    let mut clean = HashSet::new();
    for f1 in &map {
        let mut no_collide = true;
        for f2 in &map {
            if f1 == f2 {
                continue;
            }

            if f1.collides(f2) {
                clean.remove(f1);
                clean.remove(f2);
                no_collide = false;
            }
        }
        if no_collide {
            clean.insert(f1);
        }
    }

    clean.iter().next().unwrap().id
}
