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
                if a == b {
                    continue;
                }

                let positions = [*a, *b, (a.0, b.1), (a.1, b.0)];
                let mut count = 0;
                for mut m in positions {
                    let mut collisions = 0;
                    while m.0 <= max_x && m.1 <= max_y {
                        for r in &ranges {
                            if r.0.contains(&m.0) && r.1.contains(&m.1) {
                                collisions += 1;
                            }
                        }
                        m.0 += 1;
                        m.1 += 1;
                    }
                    if collisions % 2 != 0 {
                        count += 1;
                    }
                }

                if count == 4 {
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
