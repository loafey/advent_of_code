use crate::utils::{load_string, IteratorEvalExt};

fn solver(iter: impl Iterator<Item = (char, isize)>) -> isize {
    let mut colored_pieces = Vec::new();
    let mut points_between = 0;
    let (mut y, mut x) = (0, 0);
    for (dir, amount) in iter {
        colored_pieces.push((y, x));
        points_between += amount;
        match dir {
            'L' => x -= amount,
            'R' => x += amount,
            'D' => y += amount,
            'U' => y -= amount,
            _ => {}
        }
    }
    colored_pieces.push((y, x));
    ((colored_pieces
        .windows(2)
        .map(|list| {
            let [a, b] = list else { unreachable!() };
            let ((y1, x1), (y2, x2)) = (*a, *b);
            (x1 * y2) - (x2 * y1)
        })
        .sum::<isize>()
        + points_between)
        / 2)
        + 1
}

pub fn part1() -> isize {
    let input = load_string("inputs/2023/day18.input")
        .lines()
        .map(|r| {
            let mut splat = r.split_whitespace();
            let dir = splat.next().unwrap().chars().next().unwrap();
            let amount = splat.next().unwrap().parse::<isize>().unwrap();
            (dir, amount)
        })
        .eval();
    solver(input)
}

pub fn part2() -> isize {
    // Tried floodfilling first, queue computer crash :,(
    let input = load_string("inputs/2023/day18.input")
        .lines()
        .map(|r| {
            let mut splat = r.split_whitespace();
            let _dir = splat.next().unwrap().chars().next().unwrap();
            let _amount = splat.next().unwrap().parse::<i64>().unwrap();
            let s = splat
                .next()
                .unwrap()
                .chars()
                .filter(|c| !matches!(c, '(' | ')' | '#'))
                .collect::<String>();
            let dir = match &s[5..] {
                "0" => 'R',
                "1" => 'D',
                "2" => 'L',
                "3" => 'U',
                _ => unreachable!(),
            };
            let color_code = isize::from_str_radix(&s[..5], 16).unwrap();
            (dir, color_code)
        })
        .eval();
    solver(input)
}
