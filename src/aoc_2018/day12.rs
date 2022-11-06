fn parse_input() -> (Vec<char>, Vec<(Vec<char>, char)>) {
    let mut dat = include_str!("input/day12.input").split('\n');
    let initial_state = dat
        .next()
        .unwrap()
        .split_whitespace()
        .nth(2)
        .unwrap()
        .chars()
        .collect();
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

fn print_state(v: &[char]) {
    v.iter().for_each(|c| print!("{c}"));
    println!()
}

pub fn part1() -> i32 {
    let (mut state, rules) = parse_input();

    for _ in 0..20 {
        print_state(&state);
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
    print_state(&state);
    0
}

pub fn part2() -> i32 {
    0
}
