use crate::utils::{load_matrix_then, MatrixGet};

thread_local! {
    static MAP: Vec<Vec<usize>> = load_matrix_then("inputs/2023/day17.input", |c| c.to_digit(10).unwrap() as usize);
}
fn is_corner(y: usize, x: usize) -> bool {
    MAP.with(|map| y == map.len() - 1 && x == map[0].len() - 1)
}
fn get(y: usize, x: usize, ymod: isize, xmod: isize) -> Option<usize> {
    MAP.with(|map| map.matrix_get(y, x, ymod, xmod).copied())
}
fn get_neighbors(
    y: usize,
    x: usize,
    dir: Direction,
    dir_amount: usize,
) -> Vec<((usize, usize, Direction, usize), usize)> {
    let (up_amount, down_amount, left_amount, right_amount) = match dir {
        Up => (dir_amount + 1, 0, 0, 0),
        Down => (0, dir_amount + 1, 0, 0),
        Left => (0, 0, dir_amount + 1, 0),
        Right => (0, 0, 0, dir_amount + 1),
    };

    [
        get(y, x, -1, 0).map(|v| ((y - 1, x, Up, up_amount), v)),
        get(y, x, 1, 0).map(|v| ((y + 1, x, Down, down_amount), v)),
        get(y, x, 0, -1).map(|v| ((y, x - 1, Left, left_amount), v)),
        get(y, x, 0, 1).map(|v| ((y, x + 1, Right, right_amount), v)),
    ]
    .into_iter()
    .flatten()
    .filter(|((_, _, n_dir, d), _)| n_dir.not_opposite(dir) && *d < 3)
    .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
use pathfinding::directed::dijkstra::dijkstra;
use Direction::*;
impl Direction {
    fn not_opposite(self, other: Self) -> bool {
        !matches!(
            (self, other),
            (Left, Right) | (Right, Left) | (Up, Down) | (Down, Up)
        )
    }
}

pub fn part1() -> usize {
    let (_, score) = dijkstra(
        &(0, 0, Up, 0),
        |(y, x, dir, dir_a)| get_neighbors(*y, *x, *dir, *dir_a),
        |(y, x, _, _)| is_corner(*y, *x),
    )
    .unwrap_or_default();
    score
}

fn get_ultra_neighbors(
    y: usize,
    x: usize,
    dir: Direction,
    dir_amount: usize,
) -> Vec<((usize, usize, Direction, usize), usize)> {
    let (up_amount, down_amount, left_amount, right_amount) = match dir {
        Up => (dir_amount + 1, 0, 0, 0),
        Down => (0, dir_amount + 1, 0, 0),
        Left => (0, 0, dir_amount + 1, 0),
        Right => (0, 0, 0, dir_amount + 1),
    };

    [
        get(y, x, -1, 0).map(|v| ((y - 1, x, Up, up_amount), v)),
        get(y, x, 1, 0).map(|v| ((y + 1, x, Down, down_amount), v)),
        get(y, x, 0, -1).map(|v| ((y, x - 1, Left, left_amount), v)),
        get(y, x, 0, 1).map(|v| ((y, x + 1, Right, right_amount), v)),
    ]
    .into_iter()
    .flatten()
    .filter(|((_, _, a, _), _)| a.not_opposite(dir))
    // .filter(|((_, _, n_dir, d), _)| n_dir.not_opposite(dir) && *d < 3)
    .filter(|((_, _, n_dir, d), _)| {
        ((*n_dir != dir && dir_amount >= 3) || *n_dir == dir) && (*d < 10)
    })
    .collect()
}

pub fn part2() -> usize {
    dijkstra(
        &(0, 0, Right, 0),
        |(y, x, dir, dir_a)| get_ultra_neighbors(*y, *x, *dir, *dir_a),
        |(y, x, _, d)| is_corner(*y, *x) && *d >= 3,
    )
    .unwrap_or_default()
    .1
    .min(
        dijkstra(
            &(0, 0, Down, 0),
            |(y, x, dir, dir_a)| get_ultra_neighbors(*y, *x, *dir, *dir_a),
            |(y, x, _, d)| is_corner(*y, *x) && *d >= 3,
        )
        .unwrap_or_default()
        .1,
    )
}
