use crate::utils::load_string;
use std::collections::{BTreeMap, VecDeque};

#[derive(Clone, Copy, Debug)]
struct Card {
    index: usize,
    winners: usize,
}

fn parser() -> Vec<Card> {
    load_string("inputs/2023/day4.input")
        .lines()
        .map(|s| {
            let (g, cards) = s.split_once(':').unwrap();
            let index = g.split_whitespace().nth(1).unwrap().parse().unwrap();
            let (l, r) = cards.split_once('|').unwrap();
            let winners = r
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let cards = l
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>();
            let winners = cards.iter().filter(|f| winners.contains(f)).count();
            Card { index, winners }
        })
        .collect()
}

pub fn part1() -> usize {
    parser()
        .into_iter()
        .map(|c| 2usize.pow(c.winners as u32 - 1))
        .sum()
}

pub fn part2() -> usize {
    let cards = parser();
    let wins = cards
        .iter()
        .copied()
        .enumerate()
        .map(|(i, c)| {
            (c.index, {
                ((i + 1)..(i + 1 + c.winners))
                    .map(|i| cards[i])
                    .collect::<Vec<_>>()
            })
        })
        .collect::<BTreeMap<_, _>>();
    let mut res = cards.len();
    let mut cards = VecDeque::from(cards);
    while !cards.is_empty() {
        let vec = &wins[&cards.pop_front().unwrap().index];
        res += vec.len();
        vec.iter()
            .filter(|v| v.winners > 0)
            .for_each(|v| cards.push_back(*v));
    }
    res
}
