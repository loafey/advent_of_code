fn solver(window_size: usize) -> usize {
    for c in include_str!("input/day6.input")
        .chars()
        .filter(|c| !c.is_whitespace())
        .enumerate()
        .map(|(i, c)| (i + 1, c))
        .collect::<Vec<_>>()
        .windows(window_size)
    {
        let mut v = c.to_vec();
        v.sort_by_key(|(_, c)| *c);
        v.dedup_by_key(|(_, c)| *c);
        v.sort_by_key(|(i, _)| *i);
        if v.len() == window_size {
            return v.last().unwrap().0;
        }
    }
    0
}

pub fn part1() -> usize {
    solver(4)
}

pub fn part2() -> usize {
    solver(14)
}
