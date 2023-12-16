use crate::utils::{load_matrix, MatrixGet};
use std::{
    collections::{HashSet, VecDeque},
    num::Wrapping,
};

#[derive(Clone, Copy)]
enum Spot {
    Empty,
    MirrorForward,
    MirrorBackward,
    SplitterFlat,
    SplitterPipe,
}
impl From<char> for Spot {
    fn from(value: char) -> Self {
        match value {
            '.' => Empty,
            '/' => MirrorForward,
            '\\' => MirrorBackward,
            '-' => SplitterFlat,
            '|' => SplitterPipe,
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
use Direction::*;
use Spot::*;

fn solver(map: &[Vec<Spot>], start: ((usize, usize), Direction)) -> usize {
    let mut beams: VecDeque<_> = [start].into();
    // print_map(&map, &beams);
    let mut visited = HashSet::new();
    while !beams.is_empty() {
        let ((y, x), dir) = beams.get_mut(0).unwrap();
        if visited.contains(&(*y, *x, *dir)) {
            beams.pop_front();
            continue;
        }

        let mut new_stack = Vec::new();
        if let Some(p) = map.matrix_get(*y, *x, 0, 0) {
            visited.insert((*y, *x, *dir));
            match (p, *dir) {
                (Empty, _) => {}
                (MirrorForward, Left) => *dir = Down,
                (MirrorForward, Right) => *dir = Up,
                (MirrorForward, Up) => *dir = Right,
                (MirrorForward, Down) => *dir = Left,
                (MirrorBackward, Left) => *dir = Up,
                (MirrorBackward, Right) => *dir = Down,
                (MirrorBackward, Up) => *dir = Left,
                (MirrorBackward, Down) => *dir = Right,
                (SplitterFlat, Up | Down) => {
                    *dir = Left;
                    new_stack.push(((*y, *x), Right))
                }
                (SplitterFlat, Left | Right) => {}
                (SplitterPipe, Left | Right) => {
                    *dir = Up;
                    new_stack.push(((*y, *x), Down))
                }
                (SplitterPipe, Up | Down) => {}
            }
            match dir {
                Left => *x = (Wrapping(*x) - Wrapping(1_usize)).0,
                Right => *x = (Wrapping(*x) + Wrapping(1)).0,
                Up => *y = (Wrapping(*y) - Wrapping(1_usize)).0,
                Down => *y = (Wrapping(*y) + Wrapping(1)).0,
            }
            beams.rotate_left(1);
        } else {
            beams.pop_front();
        }
        new_stack.into_iter().for_each(|p| beams.push_back(p));
    }
    let unique_pos = visited
        .into_iter()
        .map(|(y, x, _)| (y, x))
        .collect::<HashSet<_>>();
    unique_pos.len()
}

pub fn part1() -> usize {
    solver(
        &load_matrix::<_, Spot>("inputs/2023/day16.input"),
        ((0, 0), Right),
    )
}

pub fn part2() -> usize {
    let map = load_matrix::<_, Spot>("inputs/2023/day16.input");
    let mut beams_start = Vec::new();
    beams_start.extend((0..map.len()).map(|y| ((y, 0), Right)));
    beams_start.extend((0..map.len()).map(|y| ((y, map[0].len() - 1), Left)));
    beams_start.extend((0..map[0].len()).map(|x| ((0, x), Down)));
    beams_start.extend((0..map[0].len()).map(|x| ((map.len() - 1, x), Up)));
    beams_start
        .into_iter()
        .map(|s| solver(&map, s))
        .max()
        .unwrap_or_default()
}
