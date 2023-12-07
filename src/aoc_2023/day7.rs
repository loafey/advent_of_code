use std::{cmp::Ordering, collections::HashMap, usize};

use crate::utils::{bi_functors::*, load_string};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Labels {
    N0,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}
fn cmp_labels(a: Labels, b: Labels, joker: bool) -> Ordering {
    if joker {}
}
impl std::fmt::Debug for Labels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::J => write!(f, "J"),
            Self::N2 => write!(f, "2"),
            Self::N3 => write!(f, "3"),
            Self::N4 => write!(f, "4"),
            Self::N5 => write!(f, "5"),
            Self::N6 => write!(f, "6"),
            Self::N7 => write!(f, "7"),
            Self::N8 => write!(f, "8"),
            Self::N9 => write!(f, "9"),
            Self::T => write!(f, "T"),
            Self::Q => write!(f, "Q"),
            Self::K => write!(f, "K"),
            Self::A => write!(f, "A"),
        }
    }
}
impl From<char> for Labels {
    fn from(value: char) -> Self {
        use Labels::*;
        match value {
            'A' => A,
            'K' => K,
            'Q' => Q,
            'J' => J,
            'T' => T,
            '9' => N9,
            '8' => N8,
            '7' => N7,
            '6' => N6,
            '5' => N5,
            '4' => N4,
            '3' => N3,
            '2' => N2,
            _ => unreachable!(),
        }
    }
}

fn biggest_rank(a: &[Labels], b: &[Labels]) -> std::cmp::Ordering {
    for (a, b) in a.iter().zip(b) {
        match a.cmp(b) {
            std::cmp::Ordering::Equal => continue,
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
    kinds.sort();
    kinds.reverse();
    match kinds {
        [5, 0, 0, 0, 0] => 6,
        [4, 1, 0, 0, 0] => 5,
        [3, 2, 0, 0, 0] => 4,
        [3, 1, 1, 0, 0] => 3,
        [2, 2, 1, 0, 0] => 2,
        [2, 1, 1, 1, 0] => 1,
        [1, 1, 1, 1, 1] => 0,
        _ => 0,
    }
}

fn most_common(cards: &[Labels]) -> Labels {
    let mut map = HashMap::new();
    for c in cards {
        if *c == Labels::J {
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
        .unwrap_or(Labels::J)
}

pub fn part1() -> usize {
    let mut input = load_string("inputs/2023/day7.input")
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
        .collect::<Vec<_>>();

    input.sort_by(|(a, _), (b, _)| match hand_type(a).cmp(&hand_type(b)) {
        std::cmp::Ordering::Equal => biggest_rank(a, b),
        x => x,
    });
    let mut tot = 0;
    for (i, (_, b)) in input.into_iter().enumerate() {
        tot += (i + 1) * b;
    }

    tot
}

pub fn part2() -> usize {
    let mut input = load_string("inputs/2023/day7.input")
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
        .collect::<Vec<_>>();

    input.sort_by(|(a, _), (b, _)| {
        let a_fix = *a;
        let a_common = most_common(&a_fix);
        let a_fix = a_fix
            .into_iter()
            .map(|c| if c == Labels::J { a_common } else { c })
            .collect::<Vec<_>>();
        // println!("{a_fix:?}: {a_common:?}");
        let b_fix = *b;
        let b_common = most_common(&b_fix);
        let b_fix = b_fix
            .into_iter()
            .map(|c| if c == Labels::J { b_common } else { c })
            .collect::<Vec<_>>();
        match hand_type(&a_fix).cmp(&hand_type(&b_fix)) {
            std::cmp::Ordering::Equal => biggest_rank(a, b),
            x => x,
        }
    });
    let mut tot = 0;
    for (i, (c, b)) in input.into_iter().enumerate() {
        println!("{} {c:?}:", i + 1);
        tot += (i + 1) * b;
    }

    tot
}
