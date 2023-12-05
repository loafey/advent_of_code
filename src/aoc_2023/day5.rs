use chumsky::primitive::Container;

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
    // seed_to_soil: Map,
    // soil_to_fertilizer: Map,
    // fertilizer_to_water: Map,
    // water_to_light: Map,
    // light_to_temperature: Map,
    // temperature_to_humidity: Map,
    // humidity_to_location: Map,
}

fn inputs() -> Inputs {
    let s = load_string("inputs/2023/day5.input");
    let mut splat = s.split("\n\n");

    let seeds = parse_row(splat.next().unwrap().split_once(':').unwrap().1);
    let seed_to_soil = parse_section(splat.next().unwrap());
    let soil_to_fertilizer = parse_section(splat.next().unwrap());
    let fertilizer_to_water = parse_section(splat.next().unwrap());
    let water_to_light = parse_section(splat.next().unwrap());
    let light_to_temperature = parse_section(splat.next().unwrap());
    let temperature_to_humidity = parse_section(splat.next().unwrap());
    let humidity_to_location = parse_section(splat.next().unwrap());

    Inputs {
        seeds,
        chain: vec![
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        ],
    }
}

pub fn part1() -> usize {
    let inputs = inputs();
    let mut res = usize::MAX;
    for v in inputs.seeds {
        let mut destination = v;
        for step in &inputs.chain {
            for (r, d) in step {
                if r.contains(&destination) {
                    destination = d + (destination - r.start);
                    break;
                }
            }
        }
        res = res.min(destination);
    }
    res
}
pub fn part2() -> usize {
    use rayon::prelude::*;
    let inputs = inputs();
    let mut seed_bag = inputs.seeds.chunks(2);
    seed_bag
        .par_bridge()
        .map(|seed_chunk| {
            let seeds = (seed_chunk[0]..seed_chunk[0] + seed_chunk[1]);
            let mut res = usize::MAX;
            for v in seeds {
                let mut destination = v;
                for step in &inputs.chain {
                    for (r, d) in step {
                        if r.contains(&destination) {
                            destination = d + (destination - r.start);
                            break;
                        }
                    }
                }
                res = res.min(destination);
            }
            res
        })
        .min()
        .unwrap_or_default()
}
