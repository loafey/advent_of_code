use rayon::prelude::*;

pub fn part1() -> i64 {
    let mut input = include_str!("../inputs/2024/day14.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut s = s
                .split(['p', '=', ',', 'v', ' '])
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i64>().unwrap());
            let p = (s.next().unwrap(), s.next().unwrap());
            let v = (s.next().unwrap(), s.next().unwrap());
            (p, v)
        })
        .collect::<Vec<_>>();

    const MAP_Y: i64 = 103;
    const MAP_X: i64 = 101;
    for _ in 0..100 {
        for ((px, py), (vx, vy)) in &mut input {
            *px = (*px + *vx).rem_euclid(MAP_X);
            *py = (*py + *vy).rem_euclid(MAP_Y);
        }
    }

    let mut q = [0, 0, 0, 0];
    let mut map = [[0; MAP_X as usize]; MAP_Y as usize];
    for ((px, py), _) in input {
        if py * 2 == MAP_Y - 1 || px * 2 == MAP_X - 1 {
            continue;
        }
        map[py as usize][px as usize] += 1;
        let x = (((px + 1) * 2) / (MAP_X + 1)) as usize;
        let y = (((py + 1) * 2) / (MAP_Y + 1)) as usize;
        match (x, y) {
            (0, 0) => q[0] += 1,
            (1, 0) => q[1] += 1,
            (0, 1) => q[2] += 1,
            (1, 1) => q[3] += 1,
            _ => panic!("{x} {y}"),
        }
    }
    q.into_iter().product()
}

pub fn part2() -> i64 {
    let mut input = include_str!("../inputs/2024/day14.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut s = s
                .split(['p', '=', ',', 'v', ' '])
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i64>().unwrap());
            let p = (s.next().unwrap(), s.next().unwrap());
            let v = (s.next().unwrap(), s.next().unwrap());
            (p, v)
        })
        .collect::<Vec<_>>();

    const MAP_Y: i64 = 103;
    const MAP_X: i64 = 101;
    const SPLIT: i64 = 7500; // this is a little cheaty, but hehe
    input.par_iter_mut().for_each(|((px, py), (vx, vy))| {
        *px = (*px + *vx * SPLIT).rem_euclid(MAP_X);
        *py = (*py + *vy * SPLIT).rem_euclid(MAP_Y);
    });
    for i in SPLIT.. {
        let mut map = [[0; MAP_X as usize]; MAP_Y as usize];
        let mut overlap = false;
        for ((px, py), (vx, vy)) in &mut input {
            *px = (*px + *vx).rem_euclid(MAP_X);
            *py = (*py + *vy).rem_euclid(MAP_Y);
            map[*py as usize][*px as usize] += 1;
            if map[*py as usize][*px as usize] > 1 {
                overlap = true;
            }
        }
        if !overlap {
            return i + 1;
        }
    }

    0
}

// 45 !
// 1 !
