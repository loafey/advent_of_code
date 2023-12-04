use std::collections::{BTreeMap, HashMap};

use crate::utils::load_string;

pub fn part1() -> usize {
    load_string("inputs/2023/day4.input")
        .lines()
        .map(|s| {
            let (_, cards) = s.split_once(':').unwrap();
            let (l, r) = cards.split_once('|').unwrap();
            let winning = r.split_whitespace().collect::<Vec<_>>();
            let mut sum = 0;
            l.split_whitespace()
                .filter(|f| winning.contains(f))
                .for_each(|_| {
                    if sum == 0 {
                        sum = 1;
                    } else {
                        sum *= 2;
                    }
                });
            sum
        })
        .sum()
}
#[derive(Clone, Copy)]
struct Card {
    index: usize,
    winners: usize,
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:\t{:?}", self.index, self.winners)
    }
}
pub fn part2() -> usize {
    let mut cards = load_string("inputs/2023/day4.input")
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
        .collect::<Vec<_>>();
    let mut i = 0;
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
    while i < cards.len() {
        let ci = cards[i].index;
        for v in &wins[&ci] {
            cards.push(*v);
        }
        i += 1;
    }
    cards.len()
}
