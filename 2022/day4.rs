use utils::load_string;
use utils::parse;
use utils::IteratorEvalExt;

fn load_input() -> impl Iterator<Item = (i32, i32, i32, i32)> {
    load_string("inputs/2022/day4.input")
        .lines()
        .map(|s| {
            let [a, b, x, y] = s.split([',', '-']).collect::<Vec<_>>()[..] else {
                unreachable!()
            };
            (parse(a), parse(b), parse(x), parse(y))
        })
        .eval()
}

pub fn part1() -> usize {
    load_input()
        .filter(|(a, b, x, y)| (a >= x && b <= y) || (x >= a && y <= b))
        .count()
}

pub fn part2() -> usize {
    load_input().filter(|(a, b, x, y)| a <= y && b >= x).count()
}
