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
            Self::Rock => write!(f, "#"),
            Self::Empty => write!(f, "."),
            //Self::Rock => write!(f, "██"),
            //Self::Empty => write!(f, ".."), //░░
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
fn check_colliding(
    dir: [isize; 2],
    coords: [usize; 2],
    box_arr: &[Box<[Spot]>],
    grid: &VecDeque<[Spot; 7]>,
) -> bool {
    let colliding = {
        let mut cols = false;
        'brum: for (y, r) in box_arr.iter().enumerate() {
            for (x, c) in r.iter().enumerate() {
                if *c != Spot::Empty
                    && grid[(coords[1] as isize + y as isize + dir[1]) as usize]
                        [(coords[0] as isize + x as isize + dir[0]) as usize]
                        == Spot::Rock
                {
                    cols = true;
                    break 'brum;
                }
            }
        }
        cols
    };
    colliding
}

pub fn part1() -> usize {
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
    let mut rock_count = 1;
    for m in input.iter().cycle() {
        let arr = current.into_arr();
        match m {
            Move::Left => {
                if coords[0] > 0 && !check_colliding([-1, 0], coords, &arr, &grid) {
                    coords[0] -= 1
                }
            }

            Move::Right => {
                if coords[0] + arr[0].len() < grid[0].len()
                    && !check_colliding([1, 0], coords, &arr, &grid)
                {
                    coords[0] += 1
                }
            }
        }
        //print_grid(&grid, coords, &arr);

        let colliding =
            coords[1] + arr.len() >= grid.len() || check_colliding([0, 1], coords, &arr, &grid);
        if !colliding {
            coords[1] += 1;
        }
        if coords[1] + arr.len() > grid.len() || colliding {
            if coords[1] + arr.len() > grid.len() {
                coords[1] -= 1;
            }
            for (y, r) in arr.iter().enumerate() {
                for (x, c) in r.iter().enumerate() {
                    if *c != Spot::Empty {
                        grid[coords[1] + y][coords[0] + x] = *c;
                    }
                }
            }
            coords = [2, 0];
            current = current.next();
            rock_count += 1;

            if let Some((height, _)) = grid
                .iter()
                .enumerate()
                .find(|(_, r)| r.iter().filter(|s| **s != Spot::Empty).count() > 0)
            {
                let arr = current.into_arr();
                // println!("{} {}", height, arr.len());
                if arr.len() + 3 >= height {
                    for _ in 0..(arr.len() + 3 - height) {
                        grid.push_front([E, E, E, E, E, E, E])
                    }
                }
                if height > arr.len() + 3 {
                    coords[1] += height - arr.len() - 3;
                }
            }

            if rock_count >= 2023 {
                return grid.len()
                    - grid
                        .iter()
                        .enumerate()
                        .find(|(_, r)| r.iter().filter(|s| **s != Spot::Empty).count() > 0)
                        .unwrap()
                        .0;
            }
        }

        //std::thread::sleep_ms(100)
    }
    //print_grid(&grid, coords, &[Box::new([])]);
    0
}

pub fn part2() -> usize {
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
    let mut rock_count = 1;
    for m in input.iter().cycle() {
        let arr = current.into_arr();
        match m {
            Move::Left => {
                if coords[0] > 0 && !check_colliding([-1, 0], coords, &arr, &grid) {
                    coords[0] -= 1
                }
            }

            Move::Right => {
                if coords[0] + arr[0].len() < grid[0].len()
                    && !check_colliding([1, 0], coords, &arr, &grid)
                {
                    coords[0] += 1
                }
            }
        }
        //print_grid(&grid, coords, &arr);

        let colliding =
            coords[1] + arr.len() >= grid.len() || check_colliding([0, 1], coords, &arr, &grid);
        if !colliding {
            coords[1] += 1;
        }
        if coords[1] + arr.len() > grid.len() || colliding {
            if coords[1] + arr.len() > grid.len() {
                coords[1] -= 1;
            }
            for (y, r) in arr.iter().enumerate() {
                for (x, c) in r.iter().enumerate() {
                    if *c != Spot::Empty {
                        grid[coords[1] + y][coords[0] + x] = *c;
                    }
                }
            }
            coords = [2, 0];
            current = current.next();
            rock_count += 1;

            if let Some((height, _)) = grid
                .iter()
                .enumerate()
                .find(|(_, r)| r.iter().filter(|s| **s != Spot::Empty).count() > 0)
            {
                let arr = current.into_arr();
                // println!("{} {}", height, arr.len());
                if arr.len() + 3 >= height {
                    for _ in 0..(arr.len() + 3 - height) {
                        grid.push_front([E, E, E, E, E, E, E])
                    }
                }
                if height > arr.len() + 3 {
                    coords[1] += height - arr.len() - 3;
                }
            }

            if rock_count >= 10000 {
                //1000000000000i64 {
                print_grid(&grid, coords, &[]);
                return grid.len()
                    - grid
                        .iter()
                        .enumerate()
                        .find(|(_, r)| r.iter().filter(|s| **s != Spot::Empty).count() > 0)
                        .unwrap()
                        .0;
            }
        }

        //std::thread::sleep_ms(100)
    }
    //print_grid(&grid, coords, &[Box::new([])]);
    0
}

fn print_grid(grid: &VecDeque<[Spot; 7]>, coords: [usize; 2], arr: &[Box<[Spot]>]) {
    //println!("\n╔══════════════╗");
    grid.iter().enumerate().for_each(|(y, r)| {
        //print!("║");
        print!("|");
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
        //println!("║")
        println!("|")
    });
    //println!("╚══════════════╝");
    println!("+-------+");
}
