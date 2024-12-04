use arrayvec::ArrayVec;

macro_rules! x {
    ($x:expr) => {
        ($x == ['X', 'M', 'A', 'S']) as i64
    };
}
macro_rules! m {
    ($x:expr) => {
        $x == ['M', 'A', 'S']
    };
}

pub fn part1() -> i64 {
    let inp = include_str!("../inputs/2024/day4.input");
    let m = inp
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect::<ArrayVec<_, 140>>())
        .collect::<ArrayVec<_, 140>>();

    let mut ans = 0;
    for (y, r) in m.iter().enumerate() {
        for (x, _) in r.iter().enumerate().filter(|(_, c)| **c == 'X') {
            if x + 3 < r.len() {
                ans += x!([m[y][x], m[y][x + 1], m[y][x + 2], m[y][x + 3]]);
            }
            if x > 2 {
                ans += x!([m[y][x], m[y][x - 1], m[y][x - 2], m[y][x - 3]]);
            }
            if y > 2 {
                ans += x!([m[y][x], m[y - 1][x], m[y - 2][x], m[y - 3][x]]);
            }
            if y + 3 < m.len() {
                ans += x!([m[y][x], m[y + 1][x], m[y + 2][x], m[y + 3][x]]);
            }
            if y > 2 && x > 2 {
                ans += x!([m[y][x], m[y - 1][x - 1], m[y - 2][x - 2], m[y - 3][x - 3]]);
            }
            if y > 2 && x + 3 < r.len() {
                ans += x!([m[y][x], m[y - 1][x + 1], m[y - 2][x + 2], m[y - 3][x + 3]]);
            }
            if y + 3 < m.len() && x > 2 {
                ans += x!([m[y][x], m[y + 1][x - 1], m[y + 2][x - 2], m[y + 3][x - 3]]);
            }
            if y + 3 < m.len() && x + 3 < r.len() {
                ans += x!([m[y][x], m[y + 1][x + 1], m[y + 2][x + 2], m[y + 3][x + 3]]);
            }
        }
    }
    ans
}

pub fn part2() -> i64 {
    let inp = include_str!("../inputs/2024/day4.input");
    let m = inp
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect::<ArrayVec<_, 140>>())
        .collect::<ArrayVec<_, 140>>();

    let mut ans = 0;
    for (y, r) in m.iter().enumerate() {
        for (x, _) in r.iter().enumerate().filter(|(_, c)| **c == 'A') {
            if x != 0 && y != 0 && x < r.len() - 1 && y < m.len() - 1 {
                let f1 = [m[y - 1][x - 1], m[y][x], m[y + 1][x + 1]];
                let f2 = [m[y + 1][x + 1], m[y][x], m[y - 1][x - 1]];
                let s1 = [m[y - 1][x + 1], m[y][x], m[y + 1][x - 1]];
                let s2 = [m[y + 1][x - 1], m[y][x], m[y - 1][x + 1]];

                ans += ((m!(f1) || m!(f2)) && (m!(s1) || m!(s2))) as i64;
            }
        }
    }
    ans
}
