use rayon::prelude::*;
use utils::MatrixGet;

fn input() -> Vec<(i64, i64)> {
    include_str!("../inputs/2025/day9.input")
        .lines()
        .map(|s| {
            let (a, b) = s.split_once(',').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

pub fn part1() -> i64 {
    let input = input();
    let mut max = 0;
    for a in &input {
        for b in &input {
            if a == b {
                continue;
            }
            let x_len = (a.0 - (b.0 + 1)).abs();
            let y_len = (a.1 - (b.1 + 1)).abs();
            max = max.max(y_len * x_len)
        }
    }
    max
}

pub fn part2() -> i64 {
    let input = input();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    let input = input
        .into_iter()
        .map(|(x, y)| (x / 100, y / 100, x, y))
        .collect::<Vec<_>>();
    for (x, y, _, _) in &input {
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
        min_x = min_x.min(*x);
        min_y = min_y.min(*y);
    }

    let mut ranges = Vec::new();
    for window in input.windows(2) {
        let (a, b) = (window[0], window[1]);
        ranges.push((
            (a.0.min(b.0))..=(a.0.max(b.0)),
            (a.1.min(b.1))..=(a.1.max(b.1)),
        ));
    }
    {
        let (a, b) = (input[input.len() - 1], input[0]);
        ranges.push((
            (a.0.min(b.0))..=(a.0.max(b.0)),
            (a.1.min(b.1))..=(a.1.max(b.1)),
        ));
    }
    let mut matrix = vec![vec!['.'; (max_x - min_x) as usize + 3]; (max_y - min_y) as usize + 3];
    for (x, y) in ranges {
        for x in x {
            for y in y.clone() {
                let x = x - min_x + 1;
                let y = y - min_y + 1;
                matrix[y as usize][x as usize] = '-';
            }
        }
    }
    let mut flood_stack = vec![(0, 0)];
    while let Some((y, x)) = flood_stack.pop() {
        matrix[y][x] = '#';
        if Some(&'.') == matrix.mget(y, x, -1, 0) {
            flood_stack.push((y - 1, x));
        }
        if Some(&'.') == matrix.mget(y, x, 1, 0) {
            flood_stack.push((y + 1, x));
        }
        if Some(&'.') == matrix.mget(y, x, 0, -1) {
            flood_stack.push((y, x - 1));
        }
        if Some(&'.') == matrix.mget(y, x, 0, 1) {
            flood_stack.push((y, x + 1));
        }
    }
    for r in &mut matrix {
        for c in r {
            if *c == '-' {
                *c = '.';
            }
        }
    }
    let mut max_value = 0;
    for a in 0..input.len() {
        println!("{a}/{}", input.len());
        let a = &input[a];
        for b in &input {
            let (mix, miy) = (a.0.min(b.0), a.1.min(b.1));
            let (max, may) = (a.0.max(b.0), a.1.max(b.1));
            let mut valid_combo = true;
            'outer: for x in mix..=max {
                for y in miy..=may {
                    if matrix[(y - min_y + 1) as usize][(x - min_x + 1) as usize] != '.' {
                        valid_combo = false;
                        break 'outer;
                    }
                }
            }
            if valid_combo {
                max_value = max_value.max(((a.2 - b.2).abs() + 1) * ((a.3 - b.3).abs() + 1))
            }
        }
    }

    max_value
}
