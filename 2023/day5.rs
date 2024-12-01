use utils::load_string;
use std::ops::Range;

type Map = Vec<(Range<usize>, usize)>;

#[derive(Debug)]
struct Inputs {
    seeds: Vec<usize>,
    chain: Vec<Map>,
}

fn parse_section(s: &str, flip: bool) -> Map {
    s.lines()
        .skip(1)
        .map(parse_row)
        .map(|v| {
            let (i1, i2) = if flip { (0, 1) } else { (1, 0) };
            (v[i1]..v[i1] + v[2], v[i2])
        })
        .collect()
}

fn parse_row(row: &str) -> Vec<usize> {
    row.split_whitespace().map(|p| p.parse().unwrap()).collect()
}

fn inputs(flip: bool) -> Inputs {
    let s = load_string("inputs/2023/day5.input");
    let mut splat = s.split("\n\n");

    let seeds = parse_row(splat.next().unwrap().split_once(':').unwrap().1);
    let chain = splat.map(|s| parse_section(s, flip)).collect();

    Inputs { seeds, chain }
}

fn find_dest(init: usize, chain: &[Map]) -> usize {
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
    let Inputs { seeds, chain } = inputs(false);
    seeds
        .into_iter()
        .map(|v| find_dest(v, &chain))
        .min()
        .unwrap_or_default()
}
pub fn part2() -> usize {
    let Inputs { seeds, mut chain } = inputs(true);
    chain.pop();
    chain.reverse();

    // Cred to Sebastian Selander for this idea, https://github.com/sebastianselander
    let seeds = seeds
        .array_chunks::<2>()
        .map(|[r1, r2]| *r1..r1 + r2)
        .collect::<Vec<_>>();
    let start = seeds.iter().map(|v| v.start).min().unwrap_or_default();

    // To improve the speed even more, we use a binary search here
    const STEP_SIZE: usize = 10000;
    let new_start = (start..)
        .step_by(STEP_SIZE)
        .find(|v| {
            let dest = find_dest(*v, &chain);
            seeds.iter().any(|r| r.contains(&dest))
        })
        .unwrap_or_default();
    (new_start - STEP_SIZE..new_start)
        .rev()
        .filter(|v| {
            let dest = find_dest(*v, &chain);
            seeds.iter().any(|r| r.contains(&dest))
        })
        .min()
        .unwrap_or_default()
}
