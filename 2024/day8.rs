use rustc_hash::{FxHashMap, FxHashSet};

const Y_MAX: usize = 50;
const X_MAX: usize = 50;

fn find_chars(mat: Map) -> FxHashMap<u8, Vec<(usize, usize)>> {
    let mut ans: FxHashMap<u8, Vec<(usize, usize)>> = FxHashMap::default();
    #[allow(clippy::needless_range_loop)]
    for y in 0..mat.len() {
        for x in 0..mat[y].len() {
            let nc = mat[y][x];
            if !matches!(nc, b'.' | b'\n') {
                ans.entry(nc).or_default().push((y, x));
            }
        }
    }
    ans
}

matrixy::matrixy!("../inputs/2024/day8.input");

pub fn part1() -> usize {
    let chars = find_chars(MAP);

    let mut posses = FxHashSet::default();
    #[allow(clippy::needless_range_loop)]
    for (_, others) in chars {
        for (y, x) in &others {
            for (ny, nx) in &others {
                if (ny, nx) == (y, x) {
                    continue;
                }
                let by = y.wrapping_add(y.wrapping_sub(*ny));
                let bx = x.wrapping_add(x.wrapping_sub(*nx));

                if by < Y_MAX && bx < X_MAX {
                    posses.insert((by, bx));
                }
            }
        }
    }

    posses.len()
}

pub fn part2() -> usize {
    let chars = find_chars(MAP);

    let mut posses = FxHashSet::default();
    #[allow(clippy::needless_range_loop)]
    for (_, others) in chars {
        for (y, x) in &others {
            posses.insert((*y, *x));
            for (ny, nx) in &others {
                if (ny, nx) == (y, x) {
                    continue;
                }
                let ydiff = y.wrapping_sub(*ny);
                let xdiff = x.wrapping_sub(*nx);
                let mut by = y.wrapping_add(ydiff);
                let mut bx = x.wrapping_add(xdiff);

                loop {
                    if by < Y_MAX && bx < X_MAX {
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

    posses.len()
}

// 368 >
