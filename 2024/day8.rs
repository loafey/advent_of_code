use std::collections::HashSet;

use rustc_hash::FxHashSet;

fn find_chars(mat: &[Vec<char>], c: char) -> Vec<(usize, usize)> {
    let mut ans = Vec::new();
    #[allow(clippy::needless_range_loop)]
    for y in 0..mat.len() {
        for x in 0..mat[y].len() {
            let nc = mat[y][x];
            if nc == c {
                ans.push((y, x));
            }
        }
    }
    ans
}

pub fn part1() -> usize {
    let mut m = include_str!("../inputs/2024/day8.input")
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let y_max = m.len();
    let x_max = m[0].len();

    let mut posses = FxHashSet::default();

    #[allow(clippy::needless_range_loop)]
    for y in 0..y_max {
        for x in 0..x_max {
            let c = m[y][x];
            if !matches!(c, '#' | '.') {
                let others = find_chars(&m, c);
                // println!("{c}: {others:?}");
                for (ny, nx) in others {
                    if (ny, nx) == (y, x) {
                        continue;
                    }

                    let by = y.wrapping_add(y.wrapping_sub(ny));
                    let bx = x.wrapping_add(x.wrapping_sub(nx));

                    if by < y_max && bx < x_max {
                        if m[by][bx] == '.' {
                            m[by][bx] = '#';
                        }
                        posses.insert((by, bx));
                    }
                }
            }
        }
    }

    for l in &m {
        for c in l {
            print!("{c}")
        }
        println!()
    }

    posses.len()
}

pub fn part2() -> usize {
    let mut m = include_str!("../inputs/2024/day8.input")
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let y_max = m.len();
    let x_max = m[0].len();

    let mut posses = FxHashSet::default();

    #[allow(clippy::needless_range_loop)]
    for y in 0..y_max {
        for x in 0..x_max {
            let c = m[y][x];
            if !matches!(c, '#' | '.') {
                posses.insert((y, x));
                let others = find_chars(&m, c);
                // println!("{c}: {others:?}");
                for (ny, nx) in others {
                    if (ny, nx) == (y, x) {
                        continue;
                    }

                    let ydiff = y.wrapping_sub(ny);
                    let xdiff = x.wrapping_sub(nx);
                    let mut by = y.wrapping_add(ydiff);
                    let mut bx = x.wrapping_add(xdiff);

                    loop {
                        println!("{by}:{bx}");
                        if by < y_max && bx < x_max {
                            if m[by][bx] == '.' {
                                m[by][bx] = '#';
                            }
                            posses.insert((by, bx));
                        } else {
                            break;
                        }
                        by = by.wrapping_add(ydiff);
                        bx = bx.wrapping_add(xdiff);
                    }
                }
            }
        }
    }

    for l in &m {
        for c in l {
            print!("{c}")
        }
        println!()
    }

    posses.len()
}

// 368 >
