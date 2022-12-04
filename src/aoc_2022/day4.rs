use crate::utils::parse;

fn load_input() -> impl Iterator<Item = (i32, i32, i32, i32)> {
    include_str!("input/day4.input").lines().map(|s| {
        let [a,b,x,y] = s.split(|c|c==','||c=='-').collect::<Vec<_>>()[..] else {unreachable!()};
        (parse(a), parse(b), parse(x), parse(y))
    })
}

pub fn part1() -> usize {
    load_input()
        .filter(|(a, b, x, y)| (a >= x && b <= y) || (x >= a && y <= b))
        .count()
}

pub fn part2() -> usize {
    load_input()
        .filter(|(a, b, x, y)| (a <= y && b >= x))
        .count()
}
