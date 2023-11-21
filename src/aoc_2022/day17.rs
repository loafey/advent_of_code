use crate::utils::load_string;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
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
    {
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
    }
}

pub fn part1() -> usize {
    let input = load_string("inputs/2022/day17.input")
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
                break;
            }
        }

        //std::thread::sleep_ms(100)
    }
    //print_grid(&grid, coords, &[Box::new([])]);
    grid.len()
        - grid
            .iter()
            .enumerate()
            .find(|(_, r)| r.iter().filter(|s| **s != Spot::Empty).count() > 0)
            .unwrap()
            .0
}

pub fn part2() -> usize {
    let input = load_string("inputs/2022/day17.input")
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
    let max = 2022; //1000000000000usize;
    let mut i = 0;
    let mut height_mod = 0;
    let mut found_loop = false;
    loop {
        let m = input[i];
        i %= input.len();
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

            if rock_count >= max {
                //1000000000000i64 {
                break;
            }

            if !found_loop {
                if let Some((index, size)) = find_loop(20, 500, &grid) {
                    //println!("{index} {size}");
                    //println!("{}", grid.len());
                    //println!("{}", max - index + size);
                    println!("{rock_count}");
                    rock_count = max - index + size;
                    println!("{rock_count}");
                    height_mod = ((max - index) / size) * size;
                    found_loop = true;
                    //println!("{}", max);
                    println!("1514285714288");
                }
            }
        }

        //std::thread::sleep_ms(100)
    }
    //print_grid(&grid, coords, &[Box::new([])]);
    grid.len()
        - grid
            .iter()
            .enumerate()
            .find(|(_, r)| r.iter().filter(|s| **s != Spot::Empty).count() > 0)
            .unwrap()
            .0
        + height_mod
}

fn find_loop<T: Eq + std::fmt::Debug>(
    start_buf_size: usize,
    start_check_size: usize,
    arr: &VecDeque<T>,
) -> Option<(usize, usize)> {
    let mut check_buf = VecDeque::new();
    let mut loop_found = false;
    let mut buf2 = VecDeque::new();
    let mut buf_size = start_buf_size;
    let check_size = start_check_size;
    for (i, val) in arr.iter().cycle().enumerate() {
        if check_size > arr.len() || check_size == start_check_size * 3 {
            break;
        }
        if i % check_size == 0 && !loop_found {
            buf_size += 1;
        }
        if buf2.len() <= buf_size {
            buf2.push_back(val);
        }
        if !loop_found && buf2.len() > buf_size {
            check_buf.push_back(buf2.pop_front().unwrap());
        } else if loop_found {
            buf2.pop_front();
        }
        if !loop_found && check_buf.len() > buf_size {
            check_buf.pop_front();
        }
        if check_buf == buf2 && !check_buf.is_empty() && !buf2.is_empty() {
            if loop_found {
                //println!("{check_buf:?} {buf2:?} collision {dif}");
                return Some((i % arr.len(), buf_size));
            }
            loop_found = true;
        }
    }
    None
}
