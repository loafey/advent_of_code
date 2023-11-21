use crate::utils::load_string;
use std::collections::VecDeque;

use crate::utils::parse_next;

fn parse_input() -> (usize, usize) {
    let binding = load_string("inputs/2018/day9.input");
    let mut split = binding.split_whitespace();
    let players = parse_next(&mut split);
    let mut split = split.skip(5);
    let points = parse_next(&mut split);

    (players, points)
}

pub fn part1() -> usize {
    let (player_amount, max_point) = parse_input();
    calc(player_amount, max_point)
}

pub fn part2() -> usize {
    let (player_amount, max_points) = parse_input();
    calc(player_amount, max_points * 100)
}
fn calc(player_amount: usize, max_points: usize) -> usize {
    let mut player_points = vec![0; player_amount];
    let mut count = 1;

    let mut marbles = VecDeque::new();
    marbles.push_front(0);

    while count < max_points {
        marbles.rotate_left(1);
        if marbles.len() > max_points * 2 {
            marbles.pop_front();
        }
        if count % 23 != 0 {
            marbles.push_back(count);
        } else {
            let point = count + marbles.remove(marbles.len() - 9).unwrap();
            marbles.rotate_right(7);

            let calc = (count - 1) % player_points.len();
            player_points[calc] += point;
        }
        count += 1;
    }
    player_points.into_iter().max().unwrap()
}
