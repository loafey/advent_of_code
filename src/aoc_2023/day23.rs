use std::collections::HashSet;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator as _};

use crate::utils::{load_matrix, MatrixGet};

#[derive(Debug, Clone, Copy)]
enum Spot {
    Forest,
    Path,
    Slope(Slope),
}

#[derive(Debug, Clone, Copy)]
enum Slope {
    Up,
    Down,
    Left,
    Right,
}
impl From<char> for Spot {
    fn from(value: char) -> Self {
        match value {
            '#' => Spot::Forest,
            '.' => Spot::Path,
            '>' => Spot::Slope(Slope::Right),
            '<' => Spot::Slope(Slope::Left),
            'v' => Spot::Slope(Slope::Down),
            '^' => Spot::Slope(Slope::Up),
            _ => unreachable!(),
        }
    }
}

fn get_neigbors(mat: &[Vec<Spot>], y: usize, x: usize) -> Vec<((usize, usize), usize)> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(|(ymod, xmod)| mat.matrix_get(y, x, ymod, xmod).map(|s| (ymod, xmod, s)))
        .filter(|(_, _, s)| !matches!(s, Spot::Forest))
        .filter(|(ymod, xmod, s)| match s {
            Spot::Slope(s) => match s {
                Slope::Up if *ymod == 1 => false,
                Slope::Down if *ymod == -1 => false,
                Slope::Left if *xmod == 1 => false,
                Slope::Right if *xmod == -1 => false,
                _ => true,
            },
            _ => true,
        })
        .map(|(ymod, xmod, _)| ((y as isize + ymod) as usize, (x as isize + xmod) as usize))
        .map(|c| (c, 1))
        .collect()
}

fn longest_path(
    mat: &[Vec<Spot>],
    mut visited: HashSet<(usize, usize)>,
    start: (usize, usize),
    end: (usize, usize),
) -> usize {
    let mut res = 0;

    let mut stack = [start].to_vec();
    while let Some((y, x)) = stack.pop() {
        if visited.contains(&(y, x)) {
            continue;
        }
        visited.insert((y, x));
        let neighbors = get_neigbors(mat, y, x)
            .into_iter()
            .filter(|c| !visited.contains(&c.0))
            .collect::<Vec<_>>();
        #[allow(clippy::comparison_chain)]
        if neighbors.len() == 1 {
            stack.push(neighbors[0].0);
        } else if neighbors.len() > 1 {
            stack.push(neighbors[0].0);
            res = res.max(
                neighbors[1..]
                    .par_iter()
                    .map(|n| longest_path(mat, visited.clone(), n.0, end))
                    .max()
                    .unwrap_or_default(),
            )
        }

        if (y, x) == end {
            break;
        }
    }

    if visited.contains(&end) {
        res = res.max(visited.len() - 1);
    }

    res
}

pub fn part1() -> usize {
    let (mat, start, end) = input();
    longest_path(&mat, HashSet::new(), start, end)
}

#[allow(clippy::type_complexity)]
fn input() -> (Vec<Vec<Spot>>, (usize, usize), (usize, usize)) {
    let mat = load_matrix::<Spot>("inputs/2023/day23.input");
    let start = mat[0]
        .iter()
        .enumerate()
        .find(|(_, s)| matches!(s, Spot::Path))
        .map(|(x, _)| (0, x))
        .unwrap();

    let end = mat[mat.len() - 1]
        .iter()
        .enumerate()
        .find(|(_, s)| matches!(s, Spot::Path))
        .map(|(x, _)| (mat.len() - 1, x))
        .unwrap();
    (mat, start, end)
}

pub fn part2() -> usize {
    let (mat, start, end) = input();
    let mat = mat
        .into_iter()
        .map(|r| {
            r.into_iter()
                .map(|s| match s {
                    Spot::Forest => Spot::Forest,
                    Spot::Path => Spot::Path,
                    Spot::Slope(_) => Spot::Path,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    longest_path(&mat, HashSet::new(), start, end)
}
