use crate::utils::load_string;
use zipperoni::Zipper2D;

fn load_input() -> Vec<Vec<u32>> {
    load_string("inputs/2022/day8.input")
        .lines()
        .map(|r| r.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

pub fn part1() -> usize {
    Zipper2D::from(&load_input())
        .filter(|(left, right, tree, up, down)| {
            left.iter().map(|i| i < *tree).all(|c| c)
                || right.iter().map(|i| i < *tree).all(|c| c)
                || up.iter().map(|i| i < tree).all(|c| c)
                || down.iter().map(|i| i < tree).all(|c| c)
        })
        .count()
}

fn get_view_len<'l, T>(tree: T, it: impl Iterator<Item = &'l T>) -> usize
where
    T: 'l + PartialEq + PartialOrd,
{
    let mut broken = false;
    it.take_while(|p| {
        if broken {
            return false;
        }
        broken = **p == tree;
        if **p > tree {
            broken = true;
            true
        } else {
            **p <= tree
        }
    })
    .count()
}

pub fn part2() -> usize {
    Zipper2D::from(&load_input())
        .map(|(left, right, tree, up, down)| {
            get_view_len(*tree, left.iter().rev())
                * get_view_len(*tree, right.iter())
                * get_view_len(*tree, up.into_iter().rev())
                * get_view_len(*tree, down.into_iter())
        })
        .max()
        .unwrap_or_default()
}
