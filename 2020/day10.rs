use memoize::memoize;

pub fn parser() -> Vec<isize> {
    let input = "16
10
15
5
1
11
7
19
6
12
4
";
    // let input = include_str!("../../inputs/2020/day10.input");
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

#[memoize]
fn solver(current: isize, input: Vec<isize>, goal: isize, res: Vec<isize>) -> isize {
    for v in &input {
        let dif = *v - current;
        let mut r = res.clone();
        r.push(*v);
        if dif > 0 && dif < 4 && solver(current + v, input.clone(), goal, r.clone()) == goal {
            return current;
        }
    }
    eprintln!("{res:?}");
    current
}

pub fn part1() -> isize {
    let adapters = parser();
    let device = adapters.iter().max().unwrap() + 3;

    solver(0, adapters, device, Vec::new());
    eprintln!("{device}");
    0
}

pub fn part2() -> usize {
    panic!()
}
