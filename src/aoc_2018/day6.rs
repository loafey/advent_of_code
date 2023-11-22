use crate::utils::load_string;
use crate::utils::parse_next;
use std::collections::BTreeSet;

fn parse_data() -> (usize, usize, Vec<(usize, usize)>) {
    let mut max_x = 0;
    let mut max_y = 0;
    let points = load_string("inputs/2018/day6.input")
        .lines()
        .map(|s| {
            let mut split = s.split(", ");
            let x = parse_next(&mut split);
            let y = parse_next(&mut split);

            if x > max_x {
                max_x = x + 2;
            }
            if y > max_y {
                max_y = y + 1;
            }

            (x, y)
        })
        .collect();
    (max_x, max_y, points)
}

pub fn part1() -> usize {
    let (max_x, max_y, points) = parse_data();

    let mut points_amount = vec![0; points.len()];
    let mut bad_points = BTreeSet::new();

    for y in 0..max_y {
        for x in 0..max_x {
            let mut closest_index = usize::MAX;
            let mut closest_distance = i32::MAX;
            let mut clash = false;
            for (i, (px, py)) in points.iter().enumerate() {
                let dist = manhattan_distance((x as i32, y as i32), (*px as i32, *py as i32));

                #[allow(clippy::comparison_chain)]
                if dist == closest_distance {
                    clash = true;
                } else if dist < closest_distance {
                    closest_index = i;
                    closest_distance = dist;
                    clash = false;
                }
            }

            if !clash {
                points_amount[closest_index] += 1;
                if x == 0 || x == max_x - 1 || y == 0 || y == max_y - 1 {
                    bad_points.insert(closest_index);
                }
            }
        }
    }

    for (modifier, b) in bad_points.into_iter().enumerate() {
        points_amount.remove(b - modifier);
    }
    points_amount.sort();
    points_amount[points_amount.len() - 1]
}

fn manhattan_distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

pub fn part2() -> usize {
    let (max_x, max_y, points) = parse_data();

    (0..max_x)
        .flat_map(|x| (0..max_y).map(move |y| (x, y)))
        .map(|(x, y)| {
            points
                .iter()
                .map(|(px, py)| manhattan_distance((x as i32, y as i32), (*px as i32, *py as i32)))
                .sum::<i32>()
        })
        .filter(|total| *total < 10000)
        .count()
}
