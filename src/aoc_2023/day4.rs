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
    let mut res = cards.len();
    let mut cards = VecDeque::from(cards);
    while !cards.is_empty() {
        let vec = &wins[&cards.pop_front().unwrap().0];
        res += vec.len();
        vec.iter()
            .filter(|v| v.1 > 0)
            .for_each(|v| cards.push_back(*v));
    }
    res
}
