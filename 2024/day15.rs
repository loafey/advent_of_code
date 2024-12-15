use utils::{FindSome, MatrixGet};

fn solve(mut map: Vec<Vec<char>>, moves: Vec<char>) -> usize {
    let (mut by, mut bx) = map
        .iter()
        .enumerate()
        .find_some(|(y, r)| {
            r.iter()
                .enumerate()
                .find_some(|(x, c)| if *c == '@' { Some((y, x)) } else { None })
        })
        .unwrap();

    for m in moves {
        let (dy, dx) = match m {
            '^' => (-1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            'v' => (1, 0),
            _ => panic!(),
        };

        let other = map.mget(by, bx, dy, dx).unwrap();
        match other {
            '.' => {
                map[by][bx] = '.';
                by = (by as isize + dy) as usize;
                bx = (bx as isize + dx) as usize;
                map[by][bx] = '@';
            }
            '[' | ']' => {
                let (ty, tx) = ((by as isize + dy) as usize, (bx as isize + dx) as usize);
                let (sy, sx) = (ty, tx);
                let mut to_move = vec![
                    (ty, tx, *other),
                    match other {
                        '[' => (ty, tx + 1, ']'),
                        ']' => (ty, tx - 1, '['),
                        _ => panic!(),
                    },
                ];
                let mut to_check = vec![(ty, tx)];
                'outer: while let Some((ty, tx)) = to_check.pop() {
                    let other = map.mget(ty, tx, 0, 0).unwrap();
                    match other {
                        '.' => {
                            for i in 0..to_check.len() {
                                let (y, x) = to_check[i];
                                if map[y][x] != '.' {
                                    to_check.insert(0, (ty, tx));
                                    continue 'outer;
                                }
                            }
                            while let Some((ty, tx, c)) = to_move.pop() {
                                let (dy, dx) =
                                    ((ty as isize + dy) as usize, (tx as isize + dx) as usize);

                                map[ty][tx] = '.';
                                map[dy][dx] = c;
                            }
                            map[by][bx] = '.';
                            map[sy][sx] = '@';
                            (by, bx) = (sy, sx);
                            break;
                        }
                        '[' | ']' => {
                            to_move.push((ty, tx, *other));
                            to_move.push(match *other {
                                '[' => (ty, tx + 1, ']'),
                                ']' => (ty, tx - 1, '['),
                                _ => panic!(),
                            });
                            to_check.insert(
                                0,
                                ((ty as isize + dy) as usize, (tx as isize + dx) as usize),
                            );
                            if dy != 0 {
                                match *other {
                                    '[' => {
                                        to_check.insert(
                                            0,
                                            (
                                                (ty as isize + dy) as usize,
                                                (tx as isize + dx + 1) as usize,
                                            ),
                                        );
                                    }
                                    ']' => {
                                        to_check.insert(
                                            0,
                                            (
                                                (ty as isize + dy) as usize,
                                                (tx as isize + dx - 1) as usize,
                                            ),
                                        );
                                    }
                                    _ => panic!(),
                                }
                            }
                        }
                        '#' => {
                            break;
                        }
                        _ => {}
                    }
                }
            }
            'O' => {
                let (mut ty, mut tx) = ((by as isize + dy) as usize, (bx as isize + dx) as usize);
                let (sy, sx) = (ty, tx);
                let mut to_move = vec![(ty, tx)];
                loop {
                    let other = map.mget(ty, tx, 0, 0).unwrap();
                    match other {
                        '.' => {
                            while let Some((ty, tx)) = to_move.pop() {
                                map[ty][tx] = '.';
                                let (ty, tx) =
                                    ((ty as isize + dy) as usize, (tx as isize + dx) as usize);
                                map[ty][tx] = 'O';
                            }
                            map[by][bx] = '.';
                            map[sy][sx] = '@';
                            (by, bx) = (sy, sx);
                            break;
                        }
                        '#' => {
                            break;
                        }
                        'O' => {
                            to_move.push((ty, tx));
                            (ty, tx) = ((ty as isize + dy) as usize, (tx as isize + dx) as usize);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    map.iter()
        .enumerate()
        .map(|(y, r)| {
            r.iter()
                .enumerate()
                .map(|(x, c)| {
                    if matches!(*c, '[' | 'O') {
                        100 * y + x
                    } else {
                        0
                    }
                })
                .filter(|s| *s > 0)
                .sum::<usize>()
        })
        .sum()
}

pub fn part1() -> usize {
    let (map, moves) = include_str!("../inputs/2024/day15.input")
        .split_once("\n\n")
        .unwrap();
    let map = map
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let moves = moves
        .lines()
        .filter(|s| !s.is_empty())
        .flat_map(|c| c.chars())
        .collect::<Vec<_>>();

    solve(map, moves)
}
pub fn part2() -> usize {
    let (map, moves) = include_str!("../inputs/2024/day15.input")
        .split_once("\n\n")
        .unwrap();
    let map = map
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.chars()
                .flat_map(|c| match c {
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '.' => ['.', '.'],
                    '@' => ['@', '.'],
                    _ => panic!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let moves = moves
        .lines()
        .filter(|s| !s.is_empty())
        .flat_map(|c| c.chars())
        .collect::<Vec<_>>();
    solve(map, moves)
}

// > 1495651
