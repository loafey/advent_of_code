use crate::utils::{load_string, BiFunctor as _, BiFunctorExt as _};
use std::collections::{BTreeMap, VecDeque};

#[derive(Clone, Copy, Debug)]
struct Card(usize, usize);

fn parser() -> Vec<Card> {
    load_string("inputs/2023/day4.input")
        .lines()
        .map(|s| {
            let s = s.split_once(':').unwrap();
            s.splat(
                |g| g.split_whitespace().nth(1).unwrap().parse().unwrap(),
                |cards| {
                    cards.split_once('|').unwrap().splot(|l, r| {
                        let winners = r
                            .split_whitespace()
                            .map(|s| s.parse::<usize>().unwrap())
                            .collect::<Vec<_>>();
                        l.split_whitespace()
                            .map(|s| s.parse().unwrap())
                            .filter(|f| winners.contains(f))
                            .count()
                    })
                },
            )
            .splot(Card)
        })
        .collect()
}

pub fn part1() -> usize {
    parser()
        .into_iter()
        .map(|c| 2usize.pow(c.1 as u32 - 1))
        .sum()
}

pub fn part2() -> usize {
    let cards = parser();
    let org_amount = cards.len();
    let wins = cards
        .iter()
        .copied()
        .enumerate()
        .map(|(i, c)| {
            (c.0, {
                ((i + 1)..(i + 1 + c.1))
                    .map(|i| cards[i])
                    .collect::<Vec<_>>()
            })
        })
        .collect::<BTreeMap<_, _>>();
    let mut card_amounts = wins.keys().map(|s| (*s, 1)).collect::<BTreeMap<_, _>>();
    for (i, card) in cards.into_iter().enumerate() {
        let wins = &wins[&(i + 1)];
        wins.iter().for_each(|c| {
            let amount = card_amounts[&(i + 1)];
            let r = card_amounts.get_mut(&c.0).unwrap();
            *r += amount;
        });
    }
    card_amounts.values().sum()
}
