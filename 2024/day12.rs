use rustc_hash::FxHashMap;
use utils::MatrixGet;
matrixy::matrixy!("../inputs/2024/day12.input");

pub fn part1() -> usize {
    let mut map = include_str!("../inputs/2024/day12.input")
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let c = map[y][x];
            if c == b'.' {
                continue;
            }
            let mut stack = vec![(y, x)];
            let mut to_remove = Vec::new();
            while let Some((y, x)) = stack.pop() {
                to_remove.push((y, x));
                if let Some(v) = map.mget(y, x, -1, 0) {
                    if *v == c && !to_remove.contains(&(y - 1, x)) {
                        stack.push((y - 1, x))
                    }
                };
                if let Some(v) = map.mget(y, x, 1, 0) {
                    if *v == c && !to_remove.contains(&(y + 1, x)) {
                        stack.push((y + 1, x))
                    }
                };
                if let Some(v) = map.mget(y, x, 0, -1) {
                    if *v == c && !to_remove.contains(&(y, x - 1)) {
                        stack.push((y, x - 1))
                    }
                };
                if let Some(v) = map.mget(y, x, 0, 1) {
                    if *v == c && !to_remove.contains(&(y, x + 1)) {
                        stack.push((y, x + 1))
                    }
                };
            }

            to_remove.sort();
            to_remove.dedup();
            let area = to_remove.len();
            let mut perimiter = FxHashMap::default();
            for (y, x) in to_remove {
                let y = y + 1;
                let x = x + 1;
                perimiter.insert((y, x), 60);
                *perimiter.entry((y - 1, x)).or_insert(0) += 1;
                *perimiter.entry((y + 1, x)).or_insert(0) += 1;
                *perimiter.entry((y, x - 1)).or_insert(0) += 1;
                *perimiter.entry((y, x + 1)).or_insert(0) += 1;
                map[y - 1][x - 1] = b'.';
            }
            let perimiter = perimiter.into_values().filter(|c| *c <= 4).sum::<usize>();
            sum += area * perimiter;
            // println!("{}: {area} | {perimiter}", c as char)
        }
    }

    sum
}
pub fn part2() -> usize {
    let mut map = MAP
        .iter()
        .flat_map(|s| {
            let s = s
                .iter()
                .filter(|c| **c != b'\n')
                .copied()
                .flat_map(|s| [s, s, s])
                .collect::<Vec<_>>();
            [s.clone(), s.clone(), s]
        })
        .collect::<Vec<_>>();
    map.iter_mut().for_each(|r| {
        r.insert(0, 0);
        r.insert(0, 0);
        r.insert(0, 0);
    });
    map.insert(0, vec![0; map[0].len()]);
    map.insert(0, vec![0; map[0].len()]);
    map.insert(0, vec![0; map[0].len()]);
    map.push(vec![0; map[0].len()]);

    let mut sum = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let c = map[y][x];
            if c == 0 {
                continue;
            }
            let mut stack = vec![(y, x)];
            let mut to_remove = Vec::new();
            while let Some((y, x)) = stack.pop() {
                to_remove.push((y, x));
                if let Some(v) = map.mget(y, x, -1, 0) {
                    if *v == c && !to_remove.contains(&(y - 1, x)) {
                        stack.push((y - 1, x))
                    }
                };
                if let Some(v) = map.mget(y, x, 1, 0) {
                    if *v == c && !to_remove.contains(&(y + 1, x)) {
                        stack.push((y + 1, x))
                    }
                };
                if let Some(v) = map.mget(y, x, 0, -1) {
                    if *v == c && !to_remove.contains(&(y, x - 1)) {
                        stack.push((y, x - 1))
                    }
                };
                if let Some(v) = map.mget(y, x, 0, 1) {
                    if *v == c && !to_remove.contains(&(y, x + 1)) {
                        stack.push((y, x + 1))
                    }
                };
            }

            to_remove.sort();
            to_remove.dedup();
            let area = to_remove.len();
            let mut perimiter = FxHashMap::default();
            let mut not_interesting = Vec::new();
            for (y, x) in to_remove {
                map[y][x] = 0;
                perimiter.insert((y, x), 0);
                not_interesting.push((y, x));
                perimiter.entry((y - 1, x)).or_insert(1);
                perimiter.entry((y + 1, x)).or_insert(1);
                perimiter.entry((y, x - 1)).or_insert(1);
                perimiter.entry((y, x + 1)).or_insert(1);
                perimiter.entry((y - 1, x - 1)).or_insert(1);
                perimiter.entry((y + 1, x - 1)).or_insert(1);
                perimiter.entry((y + 1, x + 1)).or_insert(1);
                perimiter.entry((y - 1, x + 1)).or_insert(1);
            }
            for (y, x) in not_interesting {
                perimiter.remove(&(y, x));
            }
            let mut sides = 0;

            while !perimiter.is_empty() {
                let mut keys = perimiter.keys().collect::<Vec<_>>();
                keys.sort();
                let mut stack = vec![*keys[0]];
                let (mut dy, mut dx) = (0, 1);

                while let Some((y, x)) = stack.pop() {
                    perimiter.remove(&(y, x));
                    let m = (y + dy as usize, x + dx as usize);
                    if perimiter.contains_key(&m) {
                        stack.push(m);
                    } else {
                        if dx != 0 {
                            if perimiter.contains_key(&(y - 1, x)) {
                                stack.push((y - 1, x));
                                dx = 0;
                                dy = -1;
                            } else if perimiter.contains_key(&(y + 1, x)) {
                                stack.push((y + 1, x));
                                dx = 0;
                                dy = 1;
                            }
                        } else if perimiter.contains_key(&(y, x - 1)) {
                            stack.push((y, x - 1));
                            dx = -1;
                            dy = 0;
                        } else if perimiter.contains_key(&(y, x + 1)) {
                            stack.push((y, x + 1));
                            dx = 1;
                            dy = 0;
                        }
                        sides += 1;
                    }
                }
            }

            // let y = perimiter.keys().map(|(y, _)| *y).max().unwrap() + 5;
            // let x = perimiter.keys().map(|(_, x)| *x).max().unwrap() + 5;
            // let mut map = vec![vec![0; x]; y];
            // for ((y, x), _) in perimiter {
            //     map[y][x] = 1;
            // }
            // for (i, r) in map.iter().enumerate() {
            //     println!("{i}:\t{r:?}");
            // }
            // for y in (0..map.len()).step_by(3) {
            //     for x in (0..map[y].len()).step_by(3) {
            //         if y + 2 >= map.len() || x + 2 >= map[y].len() {
            //             continue;
            //         }
            //         if matches!(
            //             [
            //                 [map[y][x], map[y][x + 1], map[y][x + 2]],
            //                 [map[y + 1][x], map[y + 1][x + 1], map[y + 1][x + 2]],
            //                 [map[y + 2][x], map[y + 2][x + 1], map[y + 2][x + 2]],
            //             ],
            //             [[0, _, _], [0, _, _], [1, 0, 0]]
            //                 | [[1, 0, 0], [0, _, _], [0, _, _]]
            //                 | [[0, 0, 1], [_, _, 0], [_, _, 0]]
            //                 | [[_, _, 0], [_, _, 0], [0, 0, 1]]
            //         ) {
            //             sides += 2;
            //         } else if matches!(
            //             [
            //                 [map[y][x], map[y][x + 1], map[y][x + 2]],
            //                 [map[y + 1][x], map[y + 1][x + 1], map[y + 1][x + 2]],
            //                 [map[y + 2][x], map[y + 2][x + 1], map[y + 2][x + 2]],
            //             ],
            //             [[1, 1, 1], [1, _, _], [1, 1, 1]]
            //                 | [[1, 1, 1], [1, _, 1], [1, _, 1]]
            //                 | [[1, 1, 1], [_, _, 1], [1, 1, 1]]
            //                 | [[1, _, 1], [1, _, 1], [1, 1, 1]]
            //         ) {
            //             sides += 1;
            //         } else if matches!(
            //             [
            //                 [map[y][x], map[y][x + 1], map[y][x + 2]],
            //                 [map[y + 1][x], map[y + 1][x + 1], map[y + 1][x + 2]],
            //                 [map[y + 2][x], map[y + 2][x + 1], map[y + 2][x + 2]],
            //             ],
            //             [[1, _, _], [1, _, _], [1, 1, 1]]
            //                 | [[1, 1, 1], [1, _, _], [1, _, _]]
            //                 | [[1, 1, 1], [_, _, 1], [_, _, 1]]
            //                 | [[_, _, 1], [_, _, 1], [1, 1, 1]]
            //         ) {
            //             sides += 1;
            //         }
            //     }
            // }
            // while !perimiter.is_empty() {
            //     let (cor, _) = perimiter.iter().next().unwrap();
            //     let cor = *cor;
            //     sides += 1;
            //     let mut stack = vec![cor];
            //     let mut up_down = None;
            //     // println!("\n{}", c as char);
            //     while let Some((y, x)) = stack.pop() {
            //         // println!("{up_down:?}");
            //         perimiter.remove(&(y, x));
            //         if perimiter.contains_key(&(y - 1, x)) && up_down.unwrap_or(true) {
            //             up_down = Some(true);
            //             stack.push((y - 1, x));
            //         } else if perimiter.contains_key(&(y, x - 1)) && !up_down.unwrap_or_default() {
            //             up_down = Some(false);
            //             stack.push((y, x - 1));
            //         }
            //         if perimiter.contains_key(&(y + 1, x)) && up_down.unwrap_or(true) {
            //             up_down = Some(true);
            //             stack.push((y + 1, x));
            //         } else if perimiter.contains_key(&(y, x + 1)) && !up_down.unwrap_or_default() {
            //             up_down = Some(false);
            //             stack.push((y, x + 1));
            //         }
            //     }
            //     // println!()
            //     // println!("{v}");
            // }
            let area = area / 9;
            // println!("{}: {area} * {sides}", c as char);
            sum += area * sides;
            // println!("{}: {area} | {perimiter}", c as char)
        }
    }

    sum
}

// 876156 >
// 946084 <
