use rayon::prelude::*;

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
    let mut ranges = Vec::new();
    for window in input.windows(2) {
        let (a, b) = (window[0], window[1]);
        ranges.push((a.0..=b.0, a.1..=b.1));
    }
    let (end_a, end_b) = (input[input.len() - 1], input[0]);
    ranges.push((end_a.0..=end_b.0, end_a.1..=end_b.1));
    let (mut max_x, mut max_y) = (0, 0);
    for (x, y) in &input {
        if *x > max_x {
            max_x = *x;
        }
        if *y > max_y {
            max_y = *y;
        }
    }

    // this is slow :))
    (0..input.len())
        .par_bridge()
        .map(|a| {
            println!("{a}/{}", input.len());
            let a = &input[a];
            let mut max = 0;
            for b in &input {
                if a == b || a.0 == b.0 || a.1 == b.1 {
                    continue;
                }

                let (min_x, max_x) = ((a.0.min(b.0)), (a.0.max(b.0)));
                let (min_y, max_y) = ((a.1.min(b.1)), (a.1.max(b.1)));
                if max_x - min_x < 2 || max_y - min_y < 2 {
                    continue;
                }

                let edges = [
                    (min_x..=max_x, min_y..=min_y),
                    (min_x..=min_x, min_y..=max_y),
                    (min_x..=max_x, max_y..=max_y),
                    (max_x..=max_x, min_y..=max_y),
                ];
                let mut ok = true;
                'outer: for (x_range, y_range) in edges {
                    for x in x_range.clone() {
                        for y in y_range.clone() {
                            for r in &ranges {
                                if r.0.contains(&x) && r.1.contains(&y) {
                                    ok = false;
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
                if ok {
                    let x_len = (a.0 - (b.0 + 1)).abs();
                    let y_len = (a.1 - (b.1 + 1)).abs();
                    max = max.max(y_len * x_len)
                }
            }
            max
        })
        .max()
        .unwrap_or_default()
}
