use crate::utils::parse;

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
    for (y, row) in trees.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            //
            if y == 0 || y == trees.len() - 1 || x == 0 || x == row.len() - 1 {
                visibles += 1;
            } else {
                let left_visible = (0..x).map(|i| trees[y][i] < *tree).all(|c| c);
                let right_visible = (x + 1..row.len()).map(|i| trees[y][i] < *tree).all(|c| c);
                let top_visible = (0..y).map(|i| trees[i][x] < *tree).all(|c| c);
                let bottom_visible = (y + 1..trees.len()).map(|i| trees[i][x] < *tree).all(|c| c);
                if left_visible || right_visible || top_visible || bottom_visible {
                    visibles += 1;
                }
            }
        }
    }
    visibles
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

    let mut max = 0;
    for (y, row) in trees.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            // left
            let mut same = false;
            let mut broken = false;
            let left = (0..x)
                .rev()
                .take_while(|p| {
                    if same || broken {
                        return false;
                    }
                    same = trees[y][*p] == *tree;
                    if trees[y][*p] > *tree {
                        broken = true;
                        true
                    } else {
                        trees[y][*p] <= *tree
                    }
                })
                .count();

            // right
            let mut same = false;
            let mut broken = false;
            let right = (x + 1..row.len())
                .take_while(|p| {
                    if same || broken {
                        return false;
                    }
                    same = trees[y][*p] == *tree;
                    if trees[y][*p] > *tree {
                        broken = true;
                        true
                    } else {
                        trees[y][*p] <= *tree
                    }
                })
                .count();

            // up
            let mut same = false;
            let mut broken = false;
            let up = (0..y)
                .rev()
                .take_while(|p| {
                    if same || broken {
                        return false;
                    }
                    same = trees[*p][x] == *tree;
                    if trees[*p][x] > *tree {
                        broken = true;
                        true
                    } else {
                        trees[*p][x] <= *tree
                    }
                })
                .count();

            // down
            let mut same = false;
            let mut broken = false;
            let down = (y + 1..trees.len())
                .take_while(|p| {
                    if same || broken {
                        return false;
                    }
                    same = trees[*p][x] == *tree;
                    if trees[*p][x] > *tree {
                        broken = true;
                        true
                    } else {
                        trees[*p][x] <= *tree
                    }
                })
                .count();

            let res = up * left * down * right;
            if res > max {
                max = res
            };
        }
    }
    max
}
