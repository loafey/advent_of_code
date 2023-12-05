use crate::utils::load_string;
use std::{collections::HashMap, ops::Range};
type Map = HashMap<Range<usize>, usize>;

fn parse_section(s: &str) -> Map {
    s.lines()
        .skip(1)
        .map(parse_row)
        .map(|v| (v[1]..v[1] + v[2], v[0]))
        .collect()
}

fn parse_row(row: &str) -> Vec<usize> {
    row.split_whitespace().map(|p| p.parse().unwrap()).collect()
}

#[derive(Debug)]
struct Inputs {
    seeds: Vec<usize>,
    chain: Vec<Map>,
}

fn inputs() -> Inputs {
    let s = load_string("inputs/2023/day5.input");
    let mut splat = s.split("\n\n");

    let seeds = parse_row(splat.next().unwrap().split_once(':').unwrap().1);
    let chain = splat.map(parse_section).collect();

    Inputs { seeds, chain }
}

fn find_dest(init: usize, chain: &Vec<Map>) -> usize {
    let mut destination = init;
    for step in chain {
        for (r, d) in step {
            if r.contains(&destination) {
                destination = d + (destination - r.start);
                break;
            }
        }
    }
    destination
}

pub fn part1() -> usize {
    let Inputs { seeds, chain } = inputs();
    seeds
        .into_iter()
        .map(|v| find_dest(v, &chain))
        .min()
        .unwrap_or_default()
}
pub fn part2() -> usize {
    use rayon::prelude::*;
    let Inputs { seeds, chain } = inputs();
    // Why write fast code when when many threads do good?
    seeds
        .par_chunks(2) // multithreading babeyyyy
        .filter_map(|seed_chunk| {
            let seeds = (seed_chunk[0]..seed_chunk[0] + seed_chunk[1]);
            seeds.map(|v| find_dest(v, &chain)).min()
        })
        .min()
        .unwrap_or_default()
}
