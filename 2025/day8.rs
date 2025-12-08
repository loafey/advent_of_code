use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}
impl Coord {
    pub fn distance(self, rhs: Self) -> f64 {
        (((self.x - rhs.x).pow(2) + (self.y - rhs.y).pow(2) + (self.z - rhs.z).pow(2)) as f64)
            .sqrt()
    }
}

fn input() -> Vec<Coord> {
    include_str!("../inputs/2025/day8.input")
        .lines()
        .map(|c| {
            let mut splut = c.split(',');
            let x = splut.next().unwrap().parse().unwrap();
            let y = splut.next().unwrap().parse().unwrap();
            let z = splut.next().unwrap().parse().unwrap();
            Coord { x, y, z }
        })
        .collect()
}

fn buckify(circuits: HashSet<(Coord, Coord)>) -> Vec<HashSet<Coord>> {
    let mut buckets: Vec<HashSet<Coord>> = Vec::new();
    let mut circuits = circuits.into_iter().collect::<Vec<_>>();
    loop {
        let mut overflow: Vec<(Coord, Coord)> = Vec::new();
        let mut found_anything = false;
        for (a, b) in circuits {
            let mut node_connected = false;
            for bucket in buckets.iter_mut() {
                if bucket.contains(&a) || bucket.contains(&b) {
                    bucket.insert(a);
                    bucket.insert(b);
                    node_connected = true;
                    found_anything = true;
                }
            }
            if !node_connected {
                overflow.push((a, b));
            }
        }
        if overflow.is_empty() {
            break;
        }
        if !found_anything {
            buckets.push(overflow.pop().map(|(a, b)| HashSet::from([a, b])).unwrap());
        }
        circuits = overflow;
    }
    buckets
}

pub fn part1() -> usize {
    let nodes = input();
    let mut circuits: HashSet<(Coord, Coord)> = HashSet::new();
    for _ in 0..1000 {
        let mut shortest = (f64::MAX, Coord::default(), Coord::default());
        for x in 0..nodes.len() {
            for y in (x + 1)..nodes.len() {
                let (x, y) = (nodes[x], nodes[y]);
                if circuits.contains(&(x, y)) {
                    continue;
                }
                let dist = x.distance(y);
                if dist < shortest.0 {
                    shortest = (dist, x, y);
                }
            }
        }
        circuits.insert((shortest.1, shortest.2));
    }

    let mut buckets = buckify(circuits);
    buckets.sort_by_key(|a| usize::MAX - a.len());
    buckets[0].len() * buckets[1].len() * buckets[2].len()
}

pub fn part2() -> i64 {
    let nodes = input();
    let mut circuits: HashSet<(Coord, Coord)> = HashSet::new();
    let mut i = 0;
    let (a, b) = loop {
        i += 1;
        let mut shortest = (f64::MAX, Coord::default(), Coord::default());
        for x in 0..nodes.len() {
            for y in (x + 1)..nodes.len() {
                let (x, y) = (nodes[x], nodes[y]);
                if circuits.contains(&(x, y)) {
                    continue;
                }
                let dist = x.distance(y);
                if dist < shortest.0 {
                    shortest = (dist, x, y);
                }
            }
        }

        if shortest.1 != Default::default() {
            circuits.insert((shortest.1, shortest.2));
        }
        let buckets = buckify(circuits.clone());
        if i % 100 == 0 {
            println!("{}", buckets.len());
        }
        if buckets.len() == 1 && buckets[0].len() == nodes.len() {
            break (shortest.1, shortest.2);
        }
    };
    a.x * b.x
}
