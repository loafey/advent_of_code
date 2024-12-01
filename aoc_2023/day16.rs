use utils::{load_matrix, MatrixGet};
use std::{collections::HashSet, num::Wrapping as W};

#[derive(Clone, Copy)]
enum Spot {
    Empty,
    MF,
    MB,
    SF,
    SP,
}
impl From<char> for Spot {
    fn from(value: char) -> Self {
        match value {
            '.' => Empty,
            '/' => MF,
            '\\' => MB,
            '-' => SF,
            '|' => SP,
            _ => unreachable!(),
        }
    }
}
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
use rayon::iter::{IntoParallelIterator, ParallelIterator as _};
use Direction::*;
use Spot::*;

fn solver(map: &[Vec<Spot>], start: ((usize, usize), Direction)) -> usize {
    let mut beams = vec![start];
    // print_map(&map, &beams);
    let mut visited = HashSet::new();
    let mut i = 0;
    while !beams.is_empty() {
        i %= beams.len();
        let ((y, x), dir) = beams.get_mut(i).unwrap();
        if visited.contains(&(*y, *x, *dir)) {
            beams.remove(i);
            continue;
        }

        let mut new_stack = Vec::new();
        if let Some(p) = map.matrix_get(*y, *x, 0, 0) {
            visited.insert((*y, *x, *dir));
            match (p, *dir) {
                (MF, Left) | (MB, Right) => *dir = Down,
                (MF, Right) | (MB, Left) => *dir = Up,
                (MF, Up) | (MB, Down) => *dir = Right,
                (MF, Down) | (MB, Up) => *dir = Left,
                (SF, Up | Down) => {
                    *dir = Left;
                    new_stack.push(((*y, *x), Right))
                }
                (SP, Left | Right) => {
                    *dir = Up;
                    new_stack.push(((*y, *x), Down))
                }
                _ => {}
            }
            match dir {
                Left => *x = (W(*x) - W(1)).0,
                Right => *x = (W(*x) + W(1)).0,
                Up => *y = (W(*y) - W(1)).0,
                Down => *y = (W(*y) + W(1)).0,
            }
            i += 1;
        } else {
            beams.remove(i);
        }
        new_stack.into_iter().for_each(|p| beams.push(p));
    }
    let unique_pos = visited
        .into_iter()
        .map(|(y, x, _)| (y, x))
        .collect::<HashSet<_>>();
    unique_pos.len()
}

pub fn part1() -> usize {
    solver(
        &load_matrix::<Spot>("inputs/2023/day16.input"),
        ((0, 0), Right),
    )
}

pub fn part2() -> usize {
    let map = load_matrix::<Spot>("inputs/2023/day16.input");
    let mut beams_start = Vec::new();
    beams_start.extend((0..map.len()).map(|y| ((y, 0), Right)));
    beams_start.extend((0..map.len()).map(|y| ((y, map[0].len() - 1), Left)));
    beams_start.extend((0..map[0].len()).map(|x| ((0, x), Down)));
    beams_start.extend((0..map[0].len()).map(|x| ((map.len() - 1, x), Up)));
    beams_start
        .into_par_iter()
        .map(|s| solver(&map, s))
        .max()
        .unwrap_or_default()
}
