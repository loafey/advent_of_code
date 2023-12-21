use std::collections::VecDeque;

use crate::utils::{load_matrix_then, MatrixGet};
use Spot::*;

#[derive(Clone, Copy)]
enum Spot {
    Empty,
    Rock,
    Start,
    Visited,
}
impl std::fmt::Debug for Spot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, " . "),
            Self::Rock => write!(f, " # "),
            Self::Start => write!(f, " S "),
            Self::Visited => write!(f, " O "),
        }
    }
}
fn print_map(map: &[Vec<Spot>]) {
    println!("──────────────────────────────── ");
    map.iter().for_each(|r| {
        r.iter().for_each(|r| print!("{r:?}"));
        println!()
    })
}
fn input() -> (Vec<Vec<Spot>>, (usize, usize)) {
    let map = load_matrix_then("inputs/2023/day21.input", |c| match c {
        '.' => Empty,
        '#' => Rock,
        'S' => Start,
        _ => unreachable!(),
    });
    let coord = map
        .iter()
        .enumerate()
        .flat_map(|(y, r)| {
            r.iter()
                .enumerate()
                .find(|(_, s)| matches!(s, Start))
                .map(|(x, _)| (y, x))
        })
        .next()
        .unwrap();
    (map, coord)
}

pub fn part1() -> usize {
    let (mut map, start) = input();
    print_map(&map);

    let mut stacks_stack = VecDeque::from([VecDeque::from([start])]);

    let mut i = 0;
    while !stacks_stack.is_empty() && i < 64 {
        let visit_stack = stacks_stack.pop_front().unwrap();
        let mut new_stack = VecDeque::new();
        for (y, x) in visit_stack {
            let neighbors = [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .into_iter()
                .filter_map(|(ymod, xmod)| {
                    map.matrix_get(y, x, ymod, xmod).copied().map(|s| {
                        (
                            s,
                            (y as isize + ymod) as usize,
                            (x as isize + xmod) as usize,
                        )
                    })
                })
                .filter(|(s, _, _)| matches!(s, Empty | Start))
                .collect::<Vec<_>>();

            map[y][x] = Empty;
            for (_, y, x) in neighbors {
                map[y][x] = Visited;
                new_stack.push_back((y, x))
            }
        }
        stacks_stack.push_back(new_stack);
        i += 1;

        // print!("{i}: ");
        // print_map(&map);
    }

    map.into_iter()
        .map(|r| r.into_iter().filter(|s| matches!(s, Visited)).count())
        .sum()
}

pub fn part2() -> usize {
    0
}
