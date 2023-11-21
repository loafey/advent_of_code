use crate::utils::load_string;
use rayon::prelude::*;

fn react_polymer(mut polymer: Vec<char>) -> usize {
    let mut i = 0;
    while i < polymer.len() - 1 {
        if polymer[i] != polymer[i + 1]
            && (polymer[i] == polymer[i + 1].to_ascii_uppercase()
                || polymer[i] == polymer[i + 1].to_ascii_lowercase())
        {
            polymer.remove(i);
            polymer.remove(i);
            i = 0;
        } else {
            i += 1;
        }
    }
    polymer.len()
}

pub fn part1() -> usize {
    react_polymer(
        load_string("inputs/2018/day5.input")
            .chars()
            .collect::<Vec<_>>(),
    )
}

pub fn part2() -> usize {
    let base = load_string("inputs/2018/day5.input");
    ('a'..='z')
        .into_par_iter()
        .map(|c| {
            react_polymer(
                base.replace([c, c.to_ascii_uppercase()], "")
                    .chars()
                    .collect::<Vec<_>>(),
            )
        })
        .reduce(
            || usize::MAX,
            |accum, current| if accum <= current { accum } else { current },
        )
}
