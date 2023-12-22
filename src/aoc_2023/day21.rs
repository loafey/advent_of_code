use std::collections::{HashMap, HashSet, VecDeque};
type Set<T> = HashSet<T>;
use crate::utils::{load_matrix_then, MatrixGet};
use Spot::*;

#[derive(Clone, Copy)]
enum Spot {
    Empty,
    Rock,
    Start,
    Visited,
}
impl std::fmt::Debug for Spot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Rock => write!(f, "#"),
            Self::Start => write!(f, "S"),
            Self::Visited => write!(f, "O"),
        }
    }
}
fn input() -> (Vec<Vec<Spot>>, (usize, usize)) {
    let map = load_matrix_then("inputs/2023/day21.input", |c| match c {
        '.' => Empty,
        '#' => Rock,
        'S' => Start,
        _ => unreachable!(),
    });
    let coord = map
        .iter()
        .enumerate()
        .flat_map(|(y, r)| {
            r.iter()
                .enumerate()
                .find(|(_, s)| matches!(s, Start))
                .map(|(x, _)| (y, x))
        })
        .next()
        .unwrap();
    (map, coord)
}

pub fn part1() -> usize {
    let (mut map, start) = input();
    // print_map(&map);

    let mut stacks_stack = VecDeque::from([VecDeque::from([start])]);

    let mut i = 0;
    while !stacks_stack.is_empty() && i < 65 {
        let visit_stack = stacks_stack.pop_front().unwrap();
        let mut new_stack = VecDeque::new();
        for (y, x) in visit_stack {
            let neighbors = [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .into_iter()
                .filter_map(|(ymod, xmod)| {
                    map.matrix_get(y, x, ymod, xmod).copied().map(|s| {
                        (
                            s,
                            (y as isize + ymod) as usize,
                            (x as isize + xmod) as usize,
                        )
                    })
                })
                .filter(|(s, _, _)| matches!(s, Empty | Start))
                .collect::<Vec<_>>();

            map[y][x] = Empty;
            for (_, y, x) in neighbors {
                map[y][x] = Visited;
                new_stack.push_back((y, x))
            }
        }
        stacks_stack.push_back(new_stack);
        i += 1;
    }

    map.iter().for_each(|r| {
        r.iter().for_each(|r| print!("{r:?}"));
        println!();
    });
    map.into_iter()
        .map(|r| r.into_iter().filter(|s| matches!(s, Visited)).count())
        .sum()
}

// const TEST_CHECK_AMOUNT: usize = 64;

// pub fn part2() -> usize {
//     // let (map, start) = input();

//     // let mut stacks_stack = VecDeque::from([Set::from([start])]);
//     // let mut visited = Set::from([start]);
//     // let mut i = 0;
//     // while !stacks_stack.is_empty() && i < TEST_CHECK_AMOUNT {
//     //     let visit_stack = stacks_stack.pop_front().unwrap();
//     //     let mut new_stack = Set::new();
//     //     for (y, x) in visit_stack {
//     //         let neighbors = [(1, 0), (-1, 0), (0, 1), (0, -1)]
//     //             .into_iter()
//     //             .map(|(ymod, xmod)| {
//     //                 (
//     //                     map.matrix_wrap(y, x, ymod, xmod),
//     //                     (y as isize + ymod) as usize,
//     //                     (x as isize + xmod) as usize,
//     //                 )
//     //             })
//     //             .filter(|(s, _, _)| matches!(s, Empty | Start))
//     //             .collect::<Vec<_>>();

//     //         visited.remove(&(y, x));
//     //         for (_, y, x) in neighbors {
//     //             visited.insert((y, x));
//     //             new_stack.insert((y, x));
//     //         }
//     //     }
//     //     stacks_stack.push_back(new_stack);
//     //     i += 1;

//     //     // print!("{i}: ");
//     //     // print_map(&map);
//     // }
//     0
// }

enum Chunk {
    NotDone {
        map: Vec<Vec<Spot>>,
        last_two: [usize; 2],
    },
    Done(usize),
}

const TEST_CHECK_AMOUNT: usize = 26501365;
pub fn part2() -> usize {
    let (map, (y, x)) = input();
    let start = (y as isize, x as isize);
    let mut chunks: HashMap<(isize, isize), Chunk> = HashMap::from([(
        (0, 0),
        Chunk::NotDone {
            map: map.clone(),
            last_two: [0, 0],
        },
    )]);
    let calc_chunk = |y, x| (y / map.len() as isize, x / map[0].len() as isize);

    let mut stacks_stack = VecDeque::from([VecDeque::from([start])]);

    let mut i = 0;
    while !stacks_stack.is_empty() && i < 64 {
        let visit_stack = stacks_stack.pop_front().unwrap();
        let mut new_stack = VecDeque::new();

        for (y, x) in visit_stack {
            let chunk_coord = calc_chunk(y, x);
            let chunk = {
                if let Some(map) = chunks.get_mut(&chunk_coord) {
                    map
                } else {
                    chunks.insert(
                        chunk_coord,
                        Chunk::NotDone {
                            map: map.clone(),
                            last_two: [0, 0],
                        },
                    );
                    let chunk = chunks.get_mut(&chunk_coord).unwrap();
                    if let Chunk::NotDone { map, .. } = chunk {
                        let cor_x = (x.rem_euclid(map[0].len() as isize)) as usize;
                        let cor_y = (y.rem_euclid(map.len() as isize)) as usize;
                        map[cor_y][cor_x] = Visited;
                    }
                    chunk
                }
            };

            if let Chunk::NotDone { map, last_two } = chunk {
                let cor_x = (x.rem_euclid(map[0].len() as isize)) as usize;
                let cor_y = (y.rem_euclid(map.len() as isize)) as usize;

                // println!("{x} {y} | {cor_x} {cor_y}");

                let mut neighbors = Vec::new();
                let mut nones = Vec::new();
                [(1, 0), (-1, 0), (0, 1), (0, -1)]
                    .into_iter()
                    .map(|(ymod, xmod)| {
                        (
                            map.matrix_get(cor_y, cor_x, ymod, xmod).copied(),
                            (cor_y as isize + ymod) as usize,
                            (cor_x as isize + xmod) as usize,
                            (y + ymod),
                            (x + xmod),
                        )
                    })
                    .for_each(|(s, cy, cx, y, x)| match s {
                        Some(s) => {
                            if matches!(s, Empty | Start) {
                                neighbors.push((cy, cx, y, x))
                            }
                        }
                        None => nones.push((y, x)),
                    });

                map[cor_y][cor_x] = Empty;
                for (cy, cx, y, x) in neighbors {
                    map[cy][cx] = Visited;
                    new_stack.push_back((y, x))
                }
                for (y, x) in nones {
                    new_stack.push_back((y, x));
                }
            }
        }
        stacks_stack.push_back(new_stack);
        i += 1;
    }

    chunks
        .into_iter()
        .map(|(i, c)| match c {
            Chunk::NotDone { map, .. } => {
                // println!("{i:?}");
                // map.iter().for_each(|r| {
                //     r.iter().for_each(|p| print!("{p:?}"));
                //     println!();
                // });
                println!();
                map.into_iter()
                    .map(|r| r.into_iter().filter(|s| matches!(s, Visited)).count())
                    .sum()
            }
            Chunk::Done(a) => a,
        })
        .sum()
}

// Works but ungodly slow
// const TEST_CHECK_AMOUNT: usize = 26501365;
// pub fn part2() -> usize {
//     let (map, (y, x)) = input();
//     let start = (y as isize, x as isize);
//     let mut chunks = HashMap::from([((0, 0), map.clone())]);

//     // print_map(&map);
//     let mut stacks_stack = VecDeque::from([Set::from([start])]);
//     let mut visited = Set::from([start]);
//     let mut i = 0;
//     while !stacks_stack.is_empty() && i < TEST_CHECK_AMOUNT {
//         let visit_stack = stacks_stack.pop_front().unwrap();
//         let mut new_stack = Set::new();
//         for (y, x) in visit_stack {
//             let neighbors = [(1, 0), (-1, 0), (0, 1), (0, -1)]
//                 .into_iter()
//                 .map(|(ymod, xmod)| {
//                     (
//                         *map.matrix_wrap(
//                             y.rem_euclid(map.len() as isize) as usize,
//                             x.rem_euclid(map[0].len() as isize) as usize,
//                             ymod,
//                             xmod,
//                         ),
//                         (y + ymod),
//                         (x + xmod),
//                     )
//                 })
//                 .filter(|(s, _, _)| matches!(s, Empty | Start))
//                 .collect::<Vec<(Spot, isize, isize)>>();
//             visited.remove(&(y, x));
//             for (_, y, x) in neighbors {
//                 visited.insert((y, x));
//                 new_stack.insert((y, x));
//             }
//         }
//         stacks_stack.push_back(new_stack);
//         i += 1;
//         // print!("{i}: ");
//         // print_map(&map);
//     }
//     visited.len()
// }
