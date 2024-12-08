use rustc_hash::{FxHashMap, FxHashSet};
use utils::bytes_to_matrix;

const Y_MAX: usize = 50;
const X_MAX: usize = 50;
type M = &'static [[u8; X_MAX + 1]; Y_MAX];

fn find_chars(mat: M) -> FxHashMap<u8, Vec<(usize, usize)>> {
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

pub fn part1() -> usize {
    let m: M = bytes_to_matrix(include_bytes!("../inputs/2024/day8.input"));
    let chars = find_chars(m);

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
    let m: M = bytes_to_matrix(include_bytes!("../inputs/2024/day8.input"));
    let chars = find_chars(m);

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
