use crate::utils::parse_next;

fn parse_input() -> (usize, usize) {
    let mut split = include_str!("input/day9.input").split_whitespace();
    let players = parse_next(&mut split);
    let mut split = split.skip(5);
    let points = parse_next(&mut split);

    (players, points)
}

pub fn part1() -> usize {
    let (player_amount, max_point) = parse_input();
    slow_calc(player_amount, max_point)
}

pub fn part2() -> usize {
    let (player_amount, max_points) = parse_input();
    {
        let mut player_points = vec![0; player_amount];
        let mut marbles = vec![0, 1];
        let mut count = 2;
        let mut current_marble = 1;
        while count <= 23 * 30 {
            if count % 23 != 0 {
                if current_marble == marbles.len() - 1 {
                    current_marble = 1;
                } else {
                    current_marble += 2;
                }
                if current_marble >= marbles.len() {
                    marbles.push(count);
                } else {
                    marbles.insert(current_marble, count);
                }
            } else {
                //println!("{:?}", marbles);
                let mut new = current_marble as i32 - 7;
                if new < 0 {
                    new += marbles.len() as i32;
                }
                let point = count + marbles.remove(new as usize);
                current_marble = new as usize;
                let calc = (count - 1) % player_points.len();
                player_points[calc] += point;
            }
            count += 1;
        }
    }

    calc(player_amount, max_points)
}
fn calc(player_amount: usize, max_points: usize) -> usize {
    let mut player_points = vec![0; player_amount];
    let mut count = 1;

    while count < max_points {
        if count % 23 == 0 {
            let magic = {
                let x = (count - 1) as f32;
                let a = 0.0888141;
                let b = 0.138934;
                let c = -6.94;
                let d = 4.414;
                a * x + b * ((c * x).rem_euclid(23.0 * d))
            }
            .round() as usize;
            let point = count + magic;
            let calc = (count - 1) % player_points.len();
            player_points[calc] += point;
        }
        count += 1;
    }
    player_points.into_iter().max().unwrap()
}

fn slow_calc(player_amount: usize, max_points: usize) -> usize {
    let mut player_points = vec![0; player_amount];
    let mut marbles = vec![0, 1];
    let mut count = 2;
    let mut current_marble = 1;
    while count <= max_points {
        if count % 23 != 0 {
            if current_marble == marbles.len() - 1 {
                current_marble = 1;
            } else {
                current_marble += 2;
            }
            if current_marble >= marbles.len() {
                marbles.push(count);
            } else {
                marbles.insert(current_marble, count);
            }
        } else {
            let mut new = current_marble as i32 - 7;
            if new < 0 {
                new += marbles.len() as i32;
            }
            let point = count + marbles.remove(new as usize);
            current_marble = new as usize;
            let calc = (count - 1) % player_points.len();
            player_points[calc] += point;
        }
        count += 1;
    }
    player_points.into_iter().max().unwrap()
}
