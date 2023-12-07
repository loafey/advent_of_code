use crate::utils::{bi_functors::*, load_string};
use std::{cmp::Ordering, collections::BTreeMap, usize};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[rustfmt::skip]
enum Labels { 
    N0, N2, N3, N4, N5, 
    N6, N7, N8, N9, T,  
    J,  Q,  K,  A,
}
use Labels::*;
impl From<char> for Labels {
    fn from(value: char) -> Self {
        #[rustfmt::skip]
        match value {
            'A' => A,  'K' => K,  'Q' => Q,  'J' => J,
            'T' => T,  '9' => N9, '8' => N8, '7' => N7,
            '6' => N6, '5' => N5, '4' => N4, '3' => N3,
            '2' => N2,  _  => unreachable!(),
        }
    }
}

fn biggest_rank(a: &[Labels], b: &[Labels], joker: bool) -> Ordering {
    for (a, b) in a.iter().zip(b) {
        if joker {
            if *a == J && *b != J {
                return Ordering::Less;
            } else if *a != J && *b == J {
                return Ordering::Greater;
            }
        }
        match a.cmp(b) {
            Ordering::Equal => continue,
            x => return x,
        }
    }
    unreachable!()
}
fn hand_type(a: &[Labels]) -> usize {
    let mut a = Vec::from(a);
    a.sort();
    a.reverse();
    let mut current = a[0];
    let mut kinds = [0; 5];
    let mut i = 0;
    for l in &a {
        if *l == current {
            kinds[i] += 1;
        } else {
            current = *l;
            i += 1;
            kinds[i] += 1;
        }
    }
    kinds.sort_by(|a, b| b.cmp(a));
    match &kinds {
        [5, 0, 0, 0, 0] => 6,
        [4, 1, 0, 0, 0] => 5,
        [3, 2, 0, 0, 0] => 4,
        [3, 1, 1, 0, 0] => 3,
        [2, 2, 1, 0, 0] => 2,
        [2, 1, 1, 1, 0] => 1,
        _ => 0,
    }
}

fn most_common(cards: &[Labels]) -> Labels {
    let mut map = BTreeMap::new();
    for c in cards {
        if matches!(c, J) {
            continue;
        } else if let Some(v) = map.get_mut(c) {
            *v += 1;
        } else {
            map.insert(c, 1);
        }
    }
    map.into_iter()
        .max_by_key(|(c, i)| *i)
        .map(|(l, _)| *l)
        .unwrap_or(J)
}

fn input() -> Vec<([Labels; 5], usize)> {
    load_string("inputs/2023/day7.input")
        .lines()
        .map(|s| {
            s.split_once(' ').unwrap().splat(
                |c| {
                    c.chars()
                        .map(Labels::from)
                        .array_chunks::<5>()
                        .next()
                        .unwrap()
                },
                |b| b.parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

fn score(input: Vec<([Labels; 5], usize)>) -> usize {
    input
        .into_iter()
        .enumerate()
        .map(|(i, (_, b))| (i + 1) * b)
        .sum()
}

fn fix_array(arr: [Labels; 5]) -> [Labels; 5] {
    let mut a_fix = arr;
    let a_common = most_common(&arr);
    a_fix.iter_mut().for_each(|c| {
        *c = if matches!(c, J) { a_common } else { *c };
    });
    a_fix
}

fn cmp(a: &[Labels], b: &[Labels], a1: &[Labels], b1: &[Labels], joker: bool) -> Ordering {
    match hand_type(a).cmp(&hand_type(b)) {
        Ordering::Equal => biggest_rank(a1, b1, joker),
        x => x,
    }
}

pub fn part1() -> usize {
    let mut input = input();
    input.sort_by(|(a, _), (b, _)| cmp(a, b, a, b, false));
    score(input)
}
pub fn part2() -> usize {
    let mut input = input();
    input.sort_by(|(a, _), (b, _)| cmp(&fix_array(*a), &fix_array(*b), a, b, true));
    score(input)
}
