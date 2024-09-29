use std::collections::BTreeSet;

fn find_seat_id(input: &str) -> i32 {
    let (mut row_start, mut row_end) = (0.0, 127.0f32);
    let (mut seat_start, mut seat_end) = (0.0, 7.0f32);
    let mut row = 0.0;
    let mut seat = 0.0;
    for c in input.chars() {
        let row_diff = ((row_end - row_start) / 2.0).round();
        let seat_diff = ((seat_end - seat_start) / 2.0).round();

        match c {
            'F' => {
                row_end -= row_diff;
                row = row_start;
            }
            'B' => {
                row_start += row_diff;
                row = row_end;
            }
            'L' => {
                seat_end -= seat_diff;
                seat = seat_start;
            }
            'R' => {
                seat_start += seat_diff;
                seat = seat_end;
            }
            _ => {}
        }
    }
    (seat + row * 8.0) as i32
}

pub fn part1() -> i32 {
    include_str!("../../inputs/2020/day5.input")
        .lines()
        .map(find_seat_id)
        .max()
        .unwrap_or_default()
}

pub fn part2() -> i32 {
    let mut set = BTreeSet::new();
    for id in include_str!("../../inputs/2020/day5.input").lines() {
        set.insert(find_seat_id(id));
    }
    let mut last = 0;
    for cur in set {
        if cur - last == 2 {
            return cur - 1;
        }
        last = cur;
    }
    0
}
