use arrayvec::ArrayVec;
use utils::MatrixGet;

#[rustfmt::skip]
macro_rules! m { ($x:expr) => { i32::from_be_bytes($x) == const { i32::from_be_bytes([0,b'M', b'A', b'S']) } } }
macro_rules! gen {
    ($ans:tt,$m:expr, $y:expr, $x:expr, $c:expr) => {
        macro_rules! index { ([ $$y:expr, $$x:expr ]) => {
            $m.mget($y, $x, $$y, $$x).copied().unwrap_or_default()
        }}
        macro_rules! ans { ($$($$k:tt)|+) => {
            let k = [$$(index!($$k)),+];
            $ans += (i32::from_be_bytes(k) == const { i32::from_be_bytes($c)}) as i64;
        }}
    };
}

matrixy::matrixy!("../inputs/2024/day4.input");

#[rustfmt::skip]
pub fn part1() -> i64 {
    let mut ans = 0;
    for (y, r) in MAP.iter().enumerate() {
        for (x, _) in r.iter().enumerate().filter(|(_, c)| **c == b'X') {
            gen!(ans, MAP, y, x, [b'X', b'M', b'A', b'S']);
            ans!([0, 0]|[ 0,  1] | [ 0,  2] | [ 0,  3]);
            ans!([0, 0]|[ 0, -1] | [ 0, -2] | [ 0, -3]);
            ans!([0, 0]|[-1,  0] | [-2,  0] | [-3,  0]);
            ans!([0, 0]|[ 1,  0] | [ 2,  0] | [ 3,  0]);
            ans!([0, 0]|[-1, -1] | [-2, -2] | [-3, -3]);
            ans!([0, 0]|[-1,  1] | [-2,  2] | [-3,  3]);
            ans!([0, 0]|[ 1, -1] | [ 2, -2] | [ 3, -3]);
            ans!([0, 0]|[ 1,  1] | [ 2,  2] | [ 3,  3]);
        }
    }
    ans
}

pub fn part2() -> i64 {
    let inp = include_str!("../inputs/2024/day4.input");
    let m = inp
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.bytes().collect::<ArrayVec<_, 140>>())
        .collect::<ArrayVec<_, 140>>();

    let mut ans = 0;
    for (y, r) in m.iter().enumerate() {
        for (x, _) in r.iter().enumerate().filter(|(_, c)| **c == b'A') {
            if x != 0 && y != 0 && x < r.len() - 1 && y < m.len() - 1 {
                let f1 = [0, m[y - 1][x - 1], m[y][x], m[y + 1][x + 1]];
                let f2 = [0, m[y + 1][x + 1], m[y][x], m[y - 1][x - 1]];
                let s1 = [0, m[y - 1][x + 1], m[y][x], m[y + 1][x - 1]];
                let s2 = [0, m[y + 1][x - 1], m[y][x], m[y - 1][x + 1]];

                ans += ((m!(f1) || m!(f2)) && (m!(s1) || m!(s2))) as i64;
            }
        }
    }
    ans
}
