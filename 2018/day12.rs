const EXTRA_LEN: usize = 70;
use utils::load_string;

fn parse_input() -> (Vec<char>, Vec<(Vec<char>, char)>) {
    let binding = load_string("inputs/2018/day12.input");
    let mut dat = binding.lines();
    let mut initial_state = dat
        .next()
        .unwrap()
        .split_whitespace()
        .nth(2)
        .unwrap()
        .chars()
        .collect::<Vec<_>>();

    for _ in 0..EXTRA_LEN {
        initial_state.insert(0, '.');
    }
    initial_state.resize(initial_state.len() + EXTRA_LEN, '.');

    dat.next();

    let rules = dat
        .map(|l| {
            let mut a = l.split_whitespace();
            let pots = a.next().unwrap().chars().collect();
            a.next();
            let result = a.next().unwrap().chars().next().unwrap();
            (pots, result)
        })
        .collect::<Vec<_>>();

    (initial_state, rules)
}

fn calc(limit: usize, mut state: Vec<char>, rules: Vec<(Vec<char>, char)>) -> i32 {
    for _ in 0..limit {
        let mut new = vec![state[0], state[1]];

        for i in 2..state.len() - 2 {
            let arr = &state[i - 2..i + 3];

            let mut passed = false;
            for (rule, c) in &rules {
                if arr == rule {
                    new.push(*c);
                    passed = true;
                }
            }
            if !passed {
                new.push('.');
            }
        }
        new.push(state[state.len() - 2]);
        new.push(state[state.len() - 1]);

        state = new;
    }

    state
        .into_iter()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .map(|(p, _)| p as i32 - (EXTRA_LEN) as i32)
        .sum()
}

pub fn part1() -> i32 {
    // println!(
    //     "{:#?}",
    //     [
    //         "...#..#.#..##......###...###..........."
    //             .chars()
    //             .filter(|c| *c == '#')
    //             .count(),
    //         "...#...#....#.....#..#..#..#..........."
    //             .chars()
    //             .filter(|c| *c == '#')
    //             .count(),
    //         "...##..##...##....#..#..#..##.........."
    //             .chars()
    //             .filter(|c| *c == '#')
    //             .count(),
    //         "..#.#...#..#.#....#..#..#...#.........."
    //             .chars()
    //             .filter(|c| *c == '#')
    //             .count(),
    //         "...#.#..#...#.#...#..#..##..##........."
    //             .chars()
    //             .filter(|c| *c == '#')
    //             .count(),
    //         "....#...##...#.#..#..#...#...#........."
    //             .chars()
    //             .filter(|c| *c == '#')
    //             .count(),
    //         "....##.#.#....#...#..##..##..##........"
    //             .chars()
    //             .filter(|c| *c == '#')
    //             .count(),
    //         "...#..###.#...##..#...#...#...#........"
    //             .chars()
    //             .filter(|c| *c == '#')
    //             .count(),
    //         "...#....##.#.#.#..##..##..##..##......."
    //             .chars()
    //             .filter(|c| *c == '#')
    //             .count(),
    //         "...##..#..#####....#...#...#...#......."
    //             .chars()
    //             .filter(|c| *c == '#')
    //             .count(),
    //         "..#.#..#...#.##....##..##..##..##......"
    //             .chars()
    //             .filter(|c| *c == '#')
    //             .count(),
    //         "...#...##...#.#...#.#...#...#...#......"
    //             .chars()
    //             .filter(|c| *c == '#')
    //             .count(),
    //         "...##.#.#....#.#...#.#..##..##..##....."
    //             .chars()
    //             .filter(|c| *c == '#')
    //             .count(),
    //         "..#..###.#....#.#...#....#...#...#....."
    //             .chars()
    //             .filter(|c| *c == '#')
    //             .count(),
    //         "..#....##.#....#.#..##...##..##..##...."
    //             .chars()
    //             .filter(|c| *c == '#')
    //             .count(),
    //         "..##..#..#.#....#....#..#.#...#...#...."
    //             .chars()
    //             .filter(|c| *c == '#')
    //             .count(),
    //         ".#.#..#...#.#...##...#...#.#..##..##..."
    //             .chars()
    //             .filter(|c| *c == '#')
    //             .count(),
    //         "..#...##...#.#.#.#...##...#....#...#..."
    //             .chars()
    //             .filter(|c| *c == '#')
    //             .count(),
    //         "..##.#.#....#####.#.#.#...##...##..##.."
    //             .chars()
    //             .filter(|c| *c == '#')
    //             .count(),
    //         ".#..###.#..#.#.#######.#.#.#..#.#...#.."
    //             .chars()
    //             .filter(|c| *c == '#')
    //             .count(),
    //         ".#....##....#####...#######....#.#..##."
    //             .chars()
    //             .filter(|c| *c == '#')
    //             .count(),
    //     ]
    // );

    let (state, rules) = parse_input();

    calc(20, state, rules)
}

pub fn part2() -> i32 {
    // let (state, rules) = parse_input();

    // calc(50000000000, state, rules)
    panic!()
}
