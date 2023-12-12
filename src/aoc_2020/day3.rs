use crate::utils::load_string;

use SlopeSpot::*;
enum SlopeSpot {
    Empty,
    Tree,
}

fn sloppy(slope: &[Vec<SlopeSpot>], movement: (usize, usize)) -> usize {
    let mut spot = (0, 0);
    let mut count = 0;
    while spot.0 < slope.len() {
        if spot != (0, 0)
            && let Tree = slope[spot.0][spot.1]
        {
            count += 1;
        }
        spot.0 += movement.0;
        spot.1 = (spot.1 + movement.1) % slope[0].len();
    }
    count
}

fn parse() -> Vec<Vec<SlopeSpot>> {
    load_string("inputs/2020/day3.input")
        .lines()
        .map(|s| {
            s.chars()
                .map(|p| match p {
                    '.' => Empty,
                    '#' => Tree,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn part1() -> usize {
    sloppy(&parse(), (1, 3))
}

pub fn part2() -> usize {
    let slope = parse();
    [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .map(|m| sloppy(&slope, m))
        .iter()
        .product()
}
