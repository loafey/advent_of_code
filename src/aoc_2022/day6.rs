use std::collections::VecDeque;

fn solver(window_size: usize) -> usize {
    let mut stack = VecDeque::new();
    for (i, c) in include_str!("input/day6.input")
        .chars()
        .filter(|c| !c.is_whitespace())
        .enumerate()
        .map(|(i, c)| (i + 1, c))
    {
        if !stack.contains(&c) {
            stack.push_front(c);
        } else {
            while stack.contains(&c) {
                stack.pop_back();
            }
            stack.push_front(c);
        }

        if stack.len() == window_size {
            println!("{stack:?}",);
            return i;
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
