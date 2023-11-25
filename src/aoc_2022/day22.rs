use crate::utils::load_string;

enum MoveInstruction {
    Walk(isize),
    Rotate(Rotate),
}
impl std::fmt::Debug for MoveInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Walk(arg0) => f.debug_tuple("W").field(arg0).finish(),
            Self::Rotate(arg0) => f.debug_tuple("R").field(arg0).finish(),
        }
    }
}

enum Rotate {
    Clockwise,
    CounterClockwise,
}
impl std::fmt::Debug for Rotate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Clockwise => write!(f, "R"),
            Self::CounterClockwise => write!(f, "L"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Object {
    Wall,
    Empty,
    TPJuice,
}
impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wall => write!(f, "██"),
            Self::Empty => write!(f, "  "),
            Self::TPJuice => write!(f, "░░"),
        }
    }
}

fn input_p1() -> (Vec<Vec<Object>>, Vec<MoveInstruction>) {
    let binding = load_string("inputs/2022/day22.input");
    let mut splat = binding.split("\n\n");
    let mut maze = splat
        .next()
        .unwrap()
        .lines()
        .map(|r| {
            r.chars()
                .map(|c| match c {
                    '#' => Object::Wall,
                    ' ' => Object::TPJuice,
                    _ => Object::Empty,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let max_size = maze.iter().map(|m| m.len()).max().unwrap();
    maze.iter_mut()
        .for_each(|m| m.append(&mut vec![Object::TPJuice; max_size - m.len()]));
    let input = {
        let mut buf = vec![String::new()];
        for c in splat.next().unwrap().chars() {
            if c.is_numeric() {
                let len = buf.len() - 1;
                buf[len].push(c);
            } else {
                buf.push(c.to_string());
                buf.push(String::new());
            }
        }
        buf.into_iter()
            .filter(|s| !s.trim().is_empty())
            .map(|s| {
                if let Ok(s) = s.parse::<isize>() {
                    MoveInstruction::Walk(s)
                } else {
                    MoveInstruction::Rotate(match &s[..] {
                        "L" => Rotate::Clockwise,
                        "R" => Rotate::CounterClockwise,
                        _ => unreachable!(),
                    })
                }
            })
            .collect::<Vec<_>>()
    };
    (maze, input)
}
#[derive(Debug, Clone, Copy)]
enum Direction {
    Left = 2,
    Right = 0,
    Up = 3,
    Down = 1,
}
impl Direction {
    fn rotate(&mut self, rot: Rotate) {
        *self = match rot {
            Rotate::Clockwise => match self {
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
            },
            Rotate::CounterClockwise => match self {
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
            },
        }
    }
}
struct Elephant {
    x: usize,
    y: usize,
    direction: Direction,
}
impl Elephant {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            direction: Direction::Right,
        }
    }
}
pub fn part1() -> usize {
    let (maze, input) = input_p1();
    let mut elephant = maze
        .iter()
        .enumerate()
        .flat_map(|(y, r)| r.iter().enumerate().map(move |(x, v)| (x, y, v)))
        .find(|(_, _, v)| **v == Object::Empty)
        .map(|(x, y, _)| Elephant::new(x, y))
        .unwrap();
    //print_maze(&elephant, &maze);
    input.into_iter().for_each(|i| match i {
        MoveInstruction::Walk(dist) => {
            for _ in 0..dist {
                let (x_mod, y_mod): (isize, isize) = match elephant.direction {
                    Direction::Left => (-1, 0),
                    Direction::Right => (1, 0),
                    Direction::Up => (0, -1),
                    Direction::Down => (0, 1),
                };
                let new_x = (elephant.x as isize + x_mod) as usize;
                let new_y = (elephant.y as isize + y_mod) as usize;
                let c = if new_y >= maze.len() || new_x >= maze[0].len() {
                    Object::TPJuice
                } else {
                    maze[new_y][new_x]
                };
                match c {
                    Object::Wall => {}
                    Object::Empty => {
                        elephant.x = new_x;
                        elephant.y = new_y;
                    }
                    Object::TPJuice => {
                        if x_mod != 0 {
                            let mut new_x = elephant.x;
                            let iter: Box<dyn Iterator<Item = _>> = if x_mod < 0 {
                                Box::new(maze[new_y].iter().enumerate().rev())
                            } else {
                                Box::new(maze[new_y].iter().enumerate())
                            };
                            for (x, c) in iter {
                                match *c {
                                    Object::Wall => break,
                                    Object::Empty => {
                                        new_x = x;
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                            elephant.x = new_x;
                        } else {
                            let mut new_y = elephant.y;
                            let iter: Box<dyn Iterator<Item = _>> = if y_mod < 0 {
                                Box::new(maze.iter().enumerate().rev())
                            } else {
                                Box::new(maze.iter().enumerate())
                            };
                            for (y, c) in iter {
                                match c[elephant.x] {
                                    Object::Wall => break,
                                    Object::Empty => {
                                        new_y = y;
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                            elephant.y = new_y;
                        }
                    }
                }
                //print_maze(&elephant, &maze);
            }
        }
        MoveInstruction::Rotate(r) => elephant.direction.rotate(r),
    });
    (elephant.x + 1) * 4 + ((elephant.y + 1) * 1000) + elephant.direction as usize
}

pub fn part2() -> i32 {
    // println!("{:?}", input_p2());
    0
}
