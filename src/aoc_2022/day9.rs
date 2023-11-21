use crate::utils::load_string;
use std::collections::HashSet;

fn is_moving_around_tail(head: [i32; 2], tail: [i32; 2], axis: usize, dif: i32) -> bool {
    let mut new = head;
    new[axis] += dif;
    is_manhattan(new, tail)
}

fn is_manhattan(a: [i32; 2], b: [i32; 2]) -> bool {
    let axis_dif = (a[0] != b[0] && a[1] != b[1]) as i32;
    ((b[0] - a[0]).abs() + (b[1] - a[1]).abs()) - axis_dif <= 1
}

fn solve<const N: usize>() -> usize {
    let start = [0, 0];
    let mut head_position = start;
    let mut tails = [start; N];
    let mut top = head_position;
    let mut visited = HashSet::new();
    load_string("inputs/2022/day9.input")
        .lines()
        .map(|r| {
            let mut split = r.split_whitespace();
            let dir = split.next().unwrap();
            let strenght = split.next().unwrap();

            (dir, strenght.parse::<i32>().unwrap())
        })
        .for_each(|ins| {
            let (move_amount, axis, dif) = match ins {
                ("R", x) => (x, 0, 1i32),
                ("L", x) => (x, 0, -1i32),
                ("D", x) => (x, 1, -1i32),
                ("U", x) => (x, 1, 1i32),
                _ => unreachable!(),
            };
            (0..move_amount).for_each(|_| {
                for i in 0..tails.len() {
                    if i == 0 {
                        if !is_moving_around_tail(head_position, tails[0], axis, dif) {
                            tails[0] = top;
                        }
                    } else if !is_manhattan(tails[i - 1], tails[i]) {
                        let x_dif = (tails[i - 1][0] - tails[i][0]).clamp(-1, 1);
                        let y_dif = (tails[i - 1][1] - tails[i][1]).clamp(-1, 1);
                        let new = [tails[i][0] + x_dif, tails[i][1] + y_dif];
                        tails[i] = new;
                    }
                    if i == N - 1 {
                        visited.insert(tails[i]);
                    }
                }
                head_position[axis] += dif;
                top = head_position;
            });
        });
    visited.len()
}

pub fn part1() -> usize {
    solve::<1>()
}

pub fn part2() -> usize {
    solve::<9>()
}
