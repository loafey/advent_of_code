use pathfinding::prelude::dijkstra;
use utils::{FindSome, MatrixGet};

pub fn part1() -> i64 {
    let mut map = include_str!("../inputs/2024/day20.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let start = map
        .iter()
        .enumerate()
        .find_some(|(y, r)| {
            r.iter()
                .enumerate()
                .find_some(|(x, c)| if *c == 'S' { Some((y, x)) } else { None })
        })
        .unwrap();
    let start = (start.0, start.1, 2);
    let end = map
        .iter()
        .enumerate()
        .find_some(|(y, r)| {
            r.iter()
                .enumerate()
                .find_some(|(x, c)| if *c == 'E' { Some((y, x)) } else { None })
        })
        .unwrap();
    println!("{start:?} {end:?}");
    let (path, len) = dijkstra(
        &start,
        |(y, x, d)| {
            let mut res = Vec::new();
            macro_rules! path {
                ($ymod:expr, $xmod:expr) => {
                    if let Some(r) = map.mget(*y, *x, $ymod, $xmod) {
                        let y = (*y as isize + $ymod) as usize;
                        let x = (*x as isize + $xmod) as usize;
                        if *r == '#' && *d > 0 {
                            res.push(((y, x, d - 1), 1));
                        } else
                        //
                        if matches!(*r, '.' | 'E' | 'S') {
                            res.push(((y, x, *d), 1));
                        }
                    }
                };
            }
            path!(-1, 0);
            path!(1, 0);
            path!(0, -1);
            path!(0, 1);
            // println!("{res:?}");
            // std::process::exit(0);
            res
        },
        |(y, x, _)| (*y, *x) == end,
    )
    .unwrap();

    println!("{path:?}");
    for (y, x, _) in path {
        map[y][x] = '|';
    }
    for r in map {
        for c in r {
            print!("{c}")
        }
        println!()
    }

    len
}

pub fn part2() -> i64 {
    0
}
