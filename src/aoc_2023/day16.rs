use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fmt::Write as _,
    io::Write as _,
    num::Wrapping,
    time::Duration,
};

#[derive(Clone, Copy)]
enum Spot {
    Empty,
    MirrorForward,
    MirrorBackward,
    SplitterFlat,
    SplitterPipe,
}
impl From<Spot> for char {
    fn from(val: Spot) -> Self {
        match val {
            Spot::Empty => ' ',
            Spot::MirrorForward => '/',
            Spot::MirrorBackward => '\\',
            Spot::SplitterFlat => '-',
            Spot::SplitterPipe => '|',
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
impl From<Direction> for char {
    fn from(val: Direction) -> Self {
        match val {
            Direction::Left => '<',
            Direction::Right => '>',
            Direction::Up => '^',
            Direction::Down => 'v',
        }
    }
}

use Direction::*;
fn print_map(map: &[Vec<Spot>], beams: &VecDeque<((usize, usize), Direction)>) -> String {
    let mut chars = vec![vec![' '; map[0].len()]; map.len()];
    map.iter().enumerate().for_each(|(y, r)| {
        r.iter()
            .enumerate()
            .for_each(|(x, s)| chars[y][x] = (*s).into());
    });
    beams.iter().for_each(|((y, x), d)| {
        if *y < map.len() && *x < map[0].len() && matches!(chars[*y][*x], ' ') {
            chars[*y][*x] = (*d).into()
        }
    });

    let mut output = String::new();
    writeln!(output, "┌{}┐", "─".repeat(map[0].len())).unwrap();
    chars.iter().for_each(|r| {
        write!(output, "│").unwrap();
        r.iter().for_each(|s| {
            write!(output, "{s}").unwrap();
        });
        writeln!(output, "│").unwrap();
    });
    writeln!(output, "└{}┘", "─".repeat(map[0].len())).unwrap();
    output
}
pub use Spot::*;

use crate::utils::{load_string, matrix_get};

pub fn part1() -> usize {
    let map = load_string("inputs/2023/day16.input")
        .lines()
        .map(|r| {
            r.chars()
                .map(|c| match c {
                    '.' => Empty,
                    '/' => MirrorForward,
                    '\\' => MirrorBackward,
                    '-' => SplitterFlat,
                    '|' => SplitterPipe,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut beams: VecDeque<_> = [((0, 0), Right)].into();
    // print_map(&map, &beams);
    let mut visited = HashSet::new();
    while !beams.is_empty() {
        let ((y, x), dir) = beams.get_mut(0).unwrap();
        if visited.contains(&(*y, *x, *dir)) {
            beams.pop_front();
            continue;
        }

        let mut new_stack = Vec::new();
        if let Some(p) = matrix_get(*y, *x, 0, 0, &map) {
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

pub fn part2() -> usize {
    let map = load_string("inputs/2023/day16.input")
        .lines()
        .map(|r| {
            r.chars()
                .map(|c| match c {
                    '.' => Empty,
                    '/' => MirrorForward,
                    '\\' => MirrorBackward,
                    '-' => SplitterFlat,
                    '|' => SplitterPipe,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut beams_start = Vec::new();
    beams_start.extend((0..map.len()).map(|y| ((y, 0), Right)));
    beams_start.extend((0..map.len()).map(|y| ((y, map[0].len() - 1), Left)));
    beams_start.extend((0..map[0].len()).map(|x| ((0, x), Down)));
    beams_start.extend((0..map[0].len()).map(|x| ((map.len() - 1, x), Up)));
    beams_start
        .into_iter()
        .map(|s| {
            let mut beams: VecDeque<_> = [s].into();
            // print_map(&map, &beams);
            let mut visited = HashSet::new();
            while !beams.is_empty() {
                let ((y, x), dir) = beams.get_mut(0).unwrap();
                if visited.contains(&(*y, *x, *dir)) {
                    beams.pop_front();
                    continue;
                }

                let mut new_stack = Vec::new();
                if let Some(p) = matrix_get(*y, *x, 0, 0, &map) {
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
        })
        .max()
        .unwrap_or_default()
}
