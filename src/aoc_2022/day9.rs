use std::collections::BTreeSet;

use itertools::Itertools;

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Down,
    Up,
}
impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "D" => Direction::Down,
            "U" => Direction::Up,
            _ => unreachable!(),
        }
    }
}

// 1 1 1
// 1 0 1
// 1 1 1
fn is_moving_around_tail(head: [i32; 2], tail: [i32; 2], axis: usize, dif: i32) -> bool {
    let mut new = head;
    new[axis] += dif;
    is_manhattan(new, tail)
}

fn is_manhattan(a: [i32; 2], b: [i32; 2]) -> bool {
    let axis_dif = (a[0] != b[0] && a[1] != b[1]) as i32;
    ((b[0] - a[0]).abs() + (b[1] - a[1]).abs()) - axis_dif <= 1
}

fn is_manhattan_cringe(a: [i32; 2], b: [i32; 2]) -> bool {
    ((b[0] - a[0]).abs() + (b[1] - a[1]).abs()) <= 1
}

pub fn part1() -> usize {
    let instructions = include_str!("input/day9.input")
        .lines()
        .map(|r| {
            let mut split = r.split_whitespace();
            let dir = split.next().unwrap();
            let strenght = split.next().unwrap();

            (Direction::from(dir), strenght.parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();

    let start = [0, 0];
    let mut head_position = start;
    let mut tail_position = start;
    let mut states = vec![(head_position, tail_position)];
    for ins in instructions {
        let (move_amount, axis, dif) = match ins {
            (Direction::Right, x) => (x, 0, 1i32),
            (Direction::Left, x) => (x, 0, -1i32),
            (Direction::Down, x) => (x, 1, -1i32),
            (Direction::Up, x) => (x, 1, 1i32),
        };
        for _ in 0..move_amount {
            if !is_moving_around_tail(head_position, tail_position, axis, dif) {
                tail_position = states[states.len() - 1].0;
            }
            head_position[axis] += dif;
            states.push((head_position, tail_position));
        }
    }
    states.into_iter().map(|(_, t)| t).unique().count()
}

pub fn part2() -> usize {
    let instructions = include_str!("input/day9.input")
        .lines()
        .map(|r| {
            let mut split = r.split_whitespace();
            let dir = split.next().unwrap();
            let strenght = split.next().unwrap();

            (Direction::from(dir), strenght.parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();

    let start = [0, 0];
    let mut head_position = start;
    let mut tails = [start; 9];

    let mut states = vec![(head_position, tails)];
    for ins in instructions {
        let (move_amount, axis, dif) = match ins {
            (Direction::Right, x) => (x, 0, 1i32),
            (Direction::Left, x) => (x, 0, -1i32),
            (Direction::Down, x) => (x, 1, -1i32),
            (Direction::Up, x) => (x, 1, 1i32),
        };
        for _ in 0..move_amount {
            let mut cache = head_position;
            let axis = axis;
            let dif = dif;
            for i in 0..tails.len() {
                if i == 0 {
                    if !is_moving_around_tail(head_position, tails[0], axis, dif) {
                        //cache = tails[0];
                        tails[0] = states[states.len() - 1].0;
                    }
                } else {
                    let before = tails[i - 1];
                    // !is_manhattan(before, tails[i]) {
                    if !is_manhattan(before, tails[i]) {
                        //std::mem::swap(&mut cache, &mut tails[i]);

                        let x_dif = (cache[0] - tails[i][0]).clamp(-1, 1);
                        let y_dif = (cache[1] - tails[i][1]).clamp(-1, 1);
                        let new = [tails[i][0] + x_dif, tails[i][1] + y_dif];
                        //if !is_manhattan(new, tails[i]) {
                        tails[i] = new;
                        cache = new;
                        //}
                    } else {
                        break;
                    }
                }
            }
            //if !is_moving_around_tail(head_position, tail_position, axis, dif) {
            //    tail_position = states[states.len() - 1].0;
            //}
            head_position[axis] += dif;
            states.push((head_position, tails));
        }
    }
    let mut visited: Vec<[i32; 2]> = Vec::new();
    for s in states.iter() {
        visited.push(s.1[s.1.len() - 1]);
    }
    visited.into_iter().unique().count()
}
