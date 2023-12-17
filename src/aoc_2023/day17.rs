use std::collections::BTreeSet;

use crate::utils::{load_matrix_then, MatrixGet};
use memoize::memoize;

thread_local! {
    static MAP: Vec<Vec<usize>> = load_matrix_then("inputs/2023/day17.input", |c| c.to_digit(10).unwrap() as usize);
}
fn is_corner(y: usize, x: usize) -> bool {
    MAP.with(|map| y == map.len() - 1 && x == map[0].len() - 1)
}
fn get(y: usize, x: usize) -> Option<usize> {
    MAP.with(|map| map.matrix_get(y, x, 0, 0).copied())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
use Direction::*;
#[derive(Eq, Clone)]
struct VisitedStack(BTreeSet<(usize, usize)>);

impl std::hash::Hash for VisitedStack {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        0.hash(state);
    }
}

impl PartialEq for VisitedStack {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

#[memoize]
fn solve_part1(
    y: usize,
    x: usize,
    dir: Direction,
    dir_amount: usize,
    visited: VisitedStack,
) -> usize {
    let mut visited = visited;
    if visited.0.contains(&(y, x)) || dir_amount > 3 {
        10000
    } else if is_corner(y, x) {
        get(y, x).unwrap()
    } else if let Some(a) = get(y, x) {
        let (up_amount, down_amount, left_amount, right_amount) = match dir {
            Up => (dir_amount + 1, 0, 0, 0),
            Down => (0, dir_amount + 1, 0, 0),
            Left => (0, 0, dir_amount + 1, 0),
            Right => (0, 0, 0, dir_amount + 1),
        };
        visited.0.insert((y, x));
        a + match dir {
            Left => [
                solve_part1(y + 1, x, Up, down_amount, visited.clone()),
                solve_part1(y - 1, x, Down, up_amount, visited.clone()),
                solve_part1(y, x - 1, Left, left_amount, visited.clone()),
            ],
            Right => [
                solve_part1(y + 1, x, Up, down_amount, visited.clone()),
                solve_part1(y - 1, x, Down, up_amount, visited.clone()),
                solve_part1(y, x + 1, Right, right_amount, visited.clone()),
            ],
            Up => [
                solve_part1(y + 1, x, Up, down_amount, visited.clone()),
                solve_part1(y, x + 1, Right, right_amount, visited.clone()),
                solve_part1(y, x - 1, Left, left_amount, visited.clone()),
            ],
            Down => [
                solve_part1(y - 1, x, Down, up_amount, visited.clone()),
                solve_part1(y, x + 1, Right, right_amount, visited.clone()),
                solve_part1(y, x - 1, Left, left_amount, visited.clone()),
            ],
        }
        .into_iter()
        .min()
        .unwrap_or(10000)
    } else {
        10000
    }
}

pub fn part1() -> usize {
    // let (y, x) = MAP.with(|m| (m.len() - 5, m[0].len() - 1));
    solve_part1(0, 1, Up, 0, VisitedStack(BTreeSet::new()))
}

pub fn part2() -> usize {
    0
}
