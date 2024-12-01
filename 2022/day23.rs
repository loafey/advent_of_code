use utils::load_string;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Object {
    Empty,
    Elf,
}
const EMPTY: Object = Object::Empty;
const ELF: Object = Object::Elf;

fn input() -> Vec<Vec<Object>> {
    load_string("inputs/2022/day23.input")
        .lines()
        .map(|r| {
            r.chars()
                .map(|m| match m {
                    '.' => Object::Empty,
                    '#' => Object::Elf,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

struct State {
    cur: usize,
    map: Vec<Vec<Object>>,
}
impl State {
    fn get_neighbors(&self, x: usize, y: usize) -> [[Object; 3]; 3] {
        let map = &self.map;
        [
            [map[y - 1][x - 1], map[y - 1][x], map[y - 1][x + 1]],
            [map[y][x - 1], EMPTY, map[y][x + 1]],
            [map[y + 1][x - 1], map[y + 1][x], map[y + 1][x + 1]],
        ]
    }

    fn get_match(&self, old_x: usize, old_y: usize) -> Option<(usize, usize)> {
        let neighbours = self.get_neighbors(old_x, old_y);
        if neighbours
            .iter()
            .flat_map(|r| r.iter())
            .filter(|c| **c == ELF)
            .count()
            == 0
        {
            return None;
        }
        Some(match self.cur {
            0 => match neighbours {
                [[EMPTY, EMPTY, EMPTY], _, _] => (old_x, old_y - 1),
                [_, _, [EMPTY, EMPTY, EMPTY]] => (old_x, old_y + 1),
                [[EMPTY, _, _], [EMPTY, _, _], [EMPTY, _, _]] => (old_x - 1, old_y),
                [[_, _, EMPTY], [_, _, EMPTY], [_, _, EMPTY]] => (old_x + 1, old_y),
                _ => (old_x, old_y),
            },
            1 => match neighbours {
                [_, _, [EMPTY, EMPTY, EMPTY]] => (old_x, old_y + 1),
                [[EMPTY, _, _], [EMPTY, _, _], [EMPTY, _, _]] => (old_x - 1, old_y),
                [[_, _, EMPTY], [_, _, EMPTY], [_, _, EMPTY]] => (old_x + 1, old_y),
                [[EMPTY, EMPTY, EMPTY], _, _] => (old_x, old_y - 1),
                _ => (old_x, old_y),
            },
            2 => match neighbours {
                [[EMPTY, _, _], [EMPTY, _, _], [EMPTY, _, _]] => (old_x - 1, old_y),
                [[_, _, EMPTY], [_, _, EMPTY], [_, _, EMPTY]] => (old_x + 1, old_y),
                [[EMPTY, EMPTY, EMPTY], _, _] => (old_x, old_y - 1),
                [_, _, [EMPTY, EMPTY, EMPTY]] => (old_x, old_y + 1),
                _ => (old_x, old_y),
            },
            3 => match neighbours {
                [[_, _, EMPTY], [_, _, EMPTY], [_, _, EMPTY]] => (old_x + 1, old_y),
                [[EMPTY, EMPTY, EMPTY], _, _] => (old_x, old_y - 1),
                [_, _, [EMPTY, EMPTY, EMPTY]] => (old_x, old_y + 1),
                [[EMPTY, _, _], [EMPTY, _, _], [EMPTY, _, _]] => (old_x - 1, old_y),
                _ => (old_x, old_y),
            },
            _ => unreachable!(),
        })
    }

    fn update(&mut self) -> bool {
        if self
            .map
            .iter()
            .enumerate()
            .flat_map(move |(y, r)| r.iter().enumerate().map(move |(x, o)| (x, y, o)))
            .filter(|(x, y, o)| {
                **o == Object::Elf
                    && (*x == 0
                        || *x == self.map[0].len() - 1
                        || *y == 0
                        || *y == self.map.len() - 1)
            })
            .count()
            > 0
        {
            self.map.iter_mut().for_each(|m| {
                m.insert(0, EMPTY);
                m.push(EMPTY);
            });
            let len = self.map[0].len();
            self.map.insert(0, vec![EMPTY; len]);
            self.map.push(vec![EMPTY; len]);
        }
        let moves = self
            .map
            .iter()
            .enumerate()
            .flat_map(|(y, r)| r.iter().enumerate().map(move |(x, o)| (x, y, o)))
            .filter(|(_, _, o)| matches!(o, Object::Elf))
            .filter_map(|(old_x, old_y, _)| {
                let (new_x, new_y) = self.get_match(old_x, old_y)?;
                Some((old_x, old_y, new_x, new_y))
            })
            .collect::<Vec<_>>();
        if moves.is_empty() {
            false
        } else {
            self.cur = (self.cur + 1) % 4;
            moves.iter().for_each(|(ox, oy, nx, ny)| {
                if !moves
                    .iter()
                    .any(|(ox2, oy2, nx2, ny2)| nx == nx2 && ny == ny2 && (ox != ox2 || oy != oy2))
                {
                    self.map[*oy][*ox] = EMPTY;
                    self.map[*ny][*nx] = ELF;
                }
            });
            true
        }
    }

    fn compress(&mut self) {
        loop {
            let clear_top = self.map[0].iter().filter(|c| **c == ELF).count() == 0;
            if clear_top {
                self.map.remove(0);
            }
            let clear_bot = self.map[self.map.len() - 1]
                .iter()
                .filter(|c| **c == ELF)
                .count()
                == 0;
            if clear_bot {
                self.map.remove(self.map.len() - 1);
            }

            let clear_left = (0..self.map.len())
                .filter(|y| self.map[*y][0] == ELF)
                .count()
                == 0;
            if clear_left {
                self.map.iter_mut().for_each(|m| {
                    m.remove(0);
                });
            }

            let clear_right = (0..self.map.len())
                .filter(|y| self.map[*y][self.map[0].len() - 1] == ELF)
                .count()
                == 0;
            if clear_right {
                self.map.iter_mut().for_each(|m| {
                    m.pop();
                });
            }
            if !(clear_bot || clear_top || clear_left || clear_right) {
                break;
            }
        }
    }
}

pub fn part1() -> usize {
    let mut state = State {
        cur: 0,
        map: input(),
    };
    for _ in 0..10 {
        state.update();
    }
    state.compress();

    state
        .map
        .into_iter()
        .flat_map(|r| r.into_iter())
        .filter(|c| *c == EMPTY)
        .count()
}

pub fn part2() -> usize {
    let mut state = State {
        cur: 0,
        map: input(),
    };
    let mut count = 1;
    while state.update() {
        count += 1;
    }

    count
}
