use pathfinding::prelude::{dijkstra, yen};
use utils::FindSome;
use Direction::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(u8)]
#[allow(unused)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn rotate_l(self) -> Self {
        let d = self as u8;
        if d == 0 {
            West
        } else {
            unsafe { std::mem::transmute::<u8, Direction>(d - 1) }
        }
    }
    fn rotate_r(self) -> Self {
        let d = self as u8;
        unsafe { std::mem::transmute::<u8, Direction>((d + 1) % 4) }
    }
}

fn find(g: char, map: &[Vec<char>]) -> (usize, usize) {
    map.iter()
        .enumerate()
        .find_some(|(y, s)| {
            s.iter()
                .enumerate()
                .find_some(|(x, c)| if *c == g { Some((y, x)) } else { None })
        })
        .unwrap()
}

pub fn part1() -> usize {
    let map = include_str!("../inputs/2024/day16.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (sy, sx) = find('S', &map);
    let (ey, ex) = find('E', &map);

    dijkstra(
        &(sy, sx, East),
        |(y, x, d)| {
            let mut r = vec![
                ((*y, *x, d.rotate_l()), 1000),
                ((*y, *x, d.rotate_r()), 1000),
            ];
            let (dy, dx) = match d {
                North => (y - 1, *x),
                East => (*y, x + 1),
                South => (y + 1, *x),
                West => (*y, x - 1),
            };
            if map[dy][dx] != '#' {
                r.push(((dy, dx, *d), 1));
            }
            r
        },
        |(y, x, _)| (*y, *x) == (ey, ex),
    )
    .unwrap_or_default()
    .1
}
pub fn part2() -> usize {
    let mut map = include_str!("../inputs/2024/day16.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (sy, sx) = find('S', &map);
    let (ey, ex) = find('E', &map);

    let mut paths = yen(
        &(sy, sx, East),
        |(y, x, d)| {
            let mut r = vec![
                ((*y, *x, d.rotate_l()), 1000),
                ((*y, *x, d.rotate_r()), 1000),
            ];
            let (dy, dx) = match d {
                North => (y - 1, *x),
                East => (*y, x + 1),
                South => (y + 1, *x),
                West => (*y, x - 1),
            };
            if map[dy][dx] != '#' {
                r.push(((dy, dx, *d), 1));
            }
            r
        },
        |(y, x, _)| (*y, *x) == (ey, ex),
        40,
    );
    paths.sort_by_key(|(_, s)| *s);
    let k = paths[0].1;
    paths.retain(|(_, s)| *s == k);

    for (p, _) in paths {
        for (y, x, _) in p {
            map[y][x] = 'O';
        }
    }

    for r in &map {
        for c in r {
            print!("{c}")
        }
        println!()
    }

    map.into_iter()
        .flat_map(|v| v.into_iter().filter(|c| *c == 'O'))
        .count()
}

// 541
// 590
