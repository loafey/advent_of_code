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

    let mut visibles = 0;
    Zipper2D::from(&trees).for_each(|(left, right, tree, up, down)| {
        if left.is_empty() || right.is_empty() || up.is_empty() || down.is_empty() {
            visibles += 1;
        } else {
            let left_visible = left.iter().map(|i| i < tree).all(|c| c);
            let right_visible = right.iter().map(|i| i < tree).all(|c| c);
            let top_visible = up.iter().map(|i| *i < tree).all(|c| c);
            let bottom_visible = down.iter().map(|i| *i < tree).all(|c| c);

            if left_visible || right_visible || top_visible || bottom_visible {
                visibles += 1;
            }
        }
    });
    visibles
}

fn get_view_len<'l, T>(tree: T, it: impl Iterator<Item = &'l T>) -> usize
where
    T: 'l + PartialEq + PartialOrd,
{
    let mut same = false;
    let mut broken = false;
    it.take_while(|p| {
        if same || broken {
            return false;
        }
        same = **p == tree;
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
            // left
            let left = get_view_len(*tree, left.iter().rev());

            // right
            let right = get_view_len(*tree, right.iter());

            // up
            let up = get_view_len(*tree, up.iter().rev().copied());

            // down
            let down = get_view_len(*tree, down.iter().copied());
            left * right * up * down
        })
        .max()
        .unwrap_or_default()
    // for (y, row) in trees.iter().enumerate() {
    //     for (x, tree) in row.iter().enumerate() {
    //         // left
    //         let mut same = false;
    //         let mut broken = false;
    //         let left = (0..x)
    //             .rev()
    //             .take_while(|p| {
    //                 if same || broken {
    //                     return false;
    //                 }
    //                 same = trees[y][*p] == *tree;
    //                 if trees[y][*p] > *tree {
    //                     broken = true;
    //                     true
    //                 } else {
    //                     trees[y][*p] <= *tree
    //                 }
    //             })
    //             .count();
    //
    //         // right
    //         let mut same = false;
    //         let mut broken = false;
    //         let right = (x + 1..row.len())
    //             .take_while(|p| {
    //                 if same || broken {
    //                     return false;
    //                 }
    //                 same = trees[y][*p] == *tree;
    //                 if trees[y][*p] > *tree {
    //                     broken = true;
    //                     true
    //                 } else {
    //                     trees[y][*p] <= *tree
    //                 }
    //             })
    //             .count();
    //
    //         // up
    //         let mut same = false;
    //         let mut broken = false;
    //         let up = (0..y)
    //             .rev()
    //             .take_while(|p| {
    //                 if same || broken {
    //                     return false;
    //                 }
    //                 same = trees[*p][x] == *tree;
    //                 if trees[*p][x] > *tree {
    //                     broken = true;
    //                     true
    //                 } else {
    //                     trees[*p][x] <= *tree
    //                 }
    //             })
    //             .count();
    //
    //         // down
    //         let mut same = false;
    //         let mut broken = false;
    //         let down = (y + 1..trees.len())
    //             .take_while(|p| {
    //                 if same || broken {
    //                     return false;
    //                 }
    //                 same = trees[*p][x] == *tree;
    //                 if trees[*p][x] > *tree {
    //                     broken = true;
    //                     true
    //                 } else {
    //                     trees[*p][x] <= *tree
    //                 }
    //             })
    //             .count();
    //
    //         let res = up * left * down * right;
    //         if res > max {
    //             max = res
    //         };
    //     }
    // }
}
