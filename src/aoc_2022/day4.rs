pub fn part1() -> usize {
    include_str!("input/day4.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            let [p1,p2] = s.split(',')
                .map(|pair| {
                    let [n1, n2] = pair.split('-').collect::<Vec<_>>()[..] else {unreachable!()};
                    (n1,n2)
                })
                .collect::<Vec<_>>()[..] else {unreachable!()};
            (p1, p2)
        })
        .map(|((a, b), (x, y))| {
            (
                (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()),
                (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()),
            )
        })
        .filter(|((a, b), (x, y))| (a >= x && b <= y) || (x >= a && y <= b))
        .count()
}

pub fn part2() -> usize {
    include_str!("input/day4.input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            let [p1,p2] = s.split(',')
            .map(|pair| {
                let [n1, n2] = pair.split('-').collect::<Vec<_>>()[..] else {unreachable!()};
                (n1,n2)
            })
            .collect::<Vec<_>>()[..] else {unreachable!()};
            (p1, p2)
        })
        .map(|((a, b), (x, y))| {
            (
                (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()),
                (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()),
            )
        })
        .filter(|((a, b), (x, y))| (a <= y && b >= x))
        .count()
}
