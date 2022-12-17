use std::collections::VecDeque;

#[derive(Debug)]
enum Move {
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Spot {
    Rock,
    Empty,
}
impl std::fmt::Debug for Spot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rock => write!(f, "██"),
            Self::Empty => write!(f, ".."), //░░
        }
    }
}

const R: Spot = Spot::Rock;
const E: Spot = Spot::Empty;

#[derive(Debug, Clone, Copy)]
enum Rocks {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
}
impl Rocks {
    fn next(self) -> Self {
        match self {
            Rocks::First => Rocks::Second,
            Rocks::Second => Rocks::Third,
            Rocks::Third => Rocks::Fourth,
            Rocks::Fourth => Rocks::Fifth,
            Rocks::Fifth => Rocks::First,
        }
    }
    fn into_arr(self) -> Box<[Box<[Spot]>]> {
        match self {
            Rocks::First => Box::new([Box::new([R, R, R, R])]),
            Rocks::Second => Box::new([
                Box::new([E, R, E]),
                Box::new([R, R, R]),
                Box::new([E, R, E]),
            ]),
            Rocks::Third => Box::new([
                Box::new([E, E, R]),
                Box::new([E, E, R]),
                Box::new([R, R, R]),
            ]),
            Rocks::Fourth => Box::new([Box::new([R]), Box::new([R]), Box::new([R]), Box::new([R])]),
            Rocks::Fifth => Box::new([Box::new([R, R]), Box::new([R, R])]),
        }
    }
}

pub fn part1() -> i32 {
    let input = include_str!("input/day17.input")
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '<' => Move::Left,
            '>' => Move::Right,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    let mut grid: VecDeque<[Spot; 7]> = vec![[E, E, E, E, E, E, E]; 4].into();
    let mut current = Rocks::First;
    let mut coords = [2, 0];
    for m in input {
        let arr = current.into_arr();
        print_grid(&grid, coords, &arr);
        match m {
            Move::Left => {
                if coords[0] > 0 {
                    coords[0] -= 1
                }
            }

            Move::Right => {
                if coords[0] + arr[0].len() < grid[0].len() {
                    coords[0] += 1
                }
            }
        }
        print_grid(&grid, coords, &arr);

        coords[1] += 1;
        let colliding = {
            let mut cols = 0;
            for (y, r) in arr.iter().enumerate() {
                let mut y_done = false;
                for (x, c) in r.iter().enumerate() {
                    if *c != Spot::Empty
                        && grid[coords[1] + y][coords[0] + x] == Spot::Rock
                        && !y_done
                    {
                        cols += 1;
                        y_done = true;
                    }
                }
            }
            cols
        };
        println!("{}", colliding);
        coords[1] -= colliding;
        if coords[1] + arr.len() >= grid.len() || colliding > 0 {
            for (y, r) in arr.iter().enumerate() {
                for (x, c) in r.iter().enumerate() {
                    if *c != Spot::Empty {
                        grid[coords[1] + y][coords[0] + x] = *c;
                    }
                }
            }
            let dif = grid.len() - (grid.len() - coords[1]);
            coords = [2, 0];
            current = current.next();
            if dif <= 3 {
                for _ in 0..(3 - dif) + current.into_arr().len() {
                    grid.push_front([Spot::Empty; 7])
                }
            }
        }

        std::thread::sleep_ms(800)
    }
    0
}

pub fn part2() -> i32 {
    0
}

fn print_grid(grid: &VecDeque<[Spot; 7]>, coords: [usize; 2], arr: &[Box<[Spot]>]) {
    println!("\n╔══════════════╗");
    grid.iter().enumerate().for_each(|(y, r)| {
        print!("║");
        r.iter().enumerate().for_each(|(x, r)| {
            if y >= coords[1]
                && y < coords[1] + arr.len()
                && x >= coords[0]
                && x < coords[0] + arr[0].len()
                && arr[y - coords[1]][x - coords[0]] != Spot::Empty
            {
                print!("{:?}", arr[y - coords[1]][x - coords[0]])
            } else {
                print!("{r:?}");
            }
        });
        println!("║")
    });
    println!("╚══════════════╝");
}
