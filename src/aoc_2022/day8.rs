use crate::utils::{parse, Zipper2D};

pub fn part1() -> usize {
    let trees = include_str!("input/day8.input")
        .lines()
        .map(|r| {
            r.split("")
                .filter(|s| !s.is_empty())
                .map(parse::<usize>)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Zipper2D::from(&trees)
        .filter(|(left, right, tree, up, down)| {
            left.iter().map(|i| i < tree).all(|c| c)
                || right.iter().map(|i| i < tree).all(|c| c)
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
    let trees = include_str!("input/day8.input")
        .lines()
        .map(|r| {
            r.split("")
                .filter(|s| !s.is_empty())
                .map(parse::<usize>)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Zipper2D::from(&trees)
        .map(|(left, right, tree, up, down)| {
            get_view_len(*tree, left.iter().rev())
                * get_view_len(*tree, right.iter())
                * get_view_len(*tree, up.into_iter().rev())
                * get_view_len(*tree, down.into_iter())
        })
        .max()
        .unwrap_or_default()
}
