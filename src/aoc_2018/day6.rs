use std::collections::HashSet;

use crate::utils::parse_next;

pub fn part1() -> usize {
    let mut max_x = 0;
    let mut max_y = 0;
    let points = include_str!("input/day6.input")
        .split('\n')
        .map(|s| {
            let mut split = s.split(", ");
            let x = parse_next::<usize>(&mut split);
            let y = parse_next::<usize>(&mut split);

            if x > max_x {
                max_x = x + 2;
            }
            if y > max_y {
                max_y = y;
            }

            (x, y)
        })
        .collect::<Vec<_>>();

    let mut points_amount = vec![0; points.len()];
    let mut bad_points = HashSet::new();

    for y in 0..max_y {
        for x in 0..max_x {
            let mut closest_index = usize::MAX;
            let mut closest_distance = i32::MAX;
            let mut clash = false;
            for (i, (px, py)) in points.iter().enumerate() {
                let dist = manhattan_distance((x as i32, y as i32), (*px as i32, *py as i32));
                if dist < closest_distance {
                    closest_index = i;
                    closest_distance = dist;
                    clash = false;
                } else if dist == closest_distance {
                    clash = true;
                }
            }

            if !clash {
                points_amount[closest_index] += 1;
                if x == 0 || x == max_x - 1 || y == 0 || y == max_y - 1 {
                    bad_points.insert(closest_index);
                }
                print!("{}", get_char(closest_index));
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!("{:?}", points_amount);
    println!("{:?}", bad_points);
    0
}

fn get_char(i: usize) -> char {
    ('a'..='z').skip(i).next().unwrap()
}

fn manhattan_distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

pub fn part2() -> usize {
    0
}
