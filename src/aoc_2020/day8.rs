pub fn parser(input: &str) -> Vec<(&str, isize, usize)> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut s = s.split_whitespace();
            let code = s.next().unwrap();
            let num = s.next().unwrap().replace("+", "").parse::<isize>().unwrap();
            (code, num, 0)
        })
        .collect::<Vec<_>>()
}

fn term_check(mut code: Vec<(&str, isize, usize)>) -> Option<isize> {
    let mut pc = 0isize;
    let mut accumulator = 0;
    while (pc as usize) < code.len() {
        let (ins, m, ex) = &mut code[pc as usize];
        *ex += 1;
        if *ex > 1 {
            return None;
        }
        match *ins {
            "acc" => {
                accumulator += *m;
                pc += 1;
            }
            "jmp" => pc += *m,
            "nop" => pc += 1,
            _ => unreachable!(),
        }
    }
    Some(accumulator)
}

pub fn part1() -> isize {
    let mut code = parser(include_str!("../../inputs/2020/day8.input"));

    let mut pc = 0isize;
    let mut accumulator = 0;
    while (pc as usize) < code.len() {
        let (ins, m, ex) = &mut code[pc as usize];
        *ex += 1;
        if *ex > 1 {
            break;
        }
        match *ins {
            "acc" => {
                accumulator += *m;
                pc += 1;
            }
            "jmp" => pc += *m,
            "nop" => pc += 1,
            _ => unreachable!(),
        }
    }

    accumulator
}

pub fn part2() -> isize {
    let code = parser(include_str!("../../inputs/2020/day8.input"));
    for i in 0..code.len() {
        match code[i].0 {
            "jmp" => {
                let mut c = code.clone();
                c[i].0 = "nop";
                
                if let Some(r) =ö term_check(c) {
                    return r;
                }
            }
            "nop" => {
                let mut c = code.clone();
                c[i].0 = "jmp";
                if let Some(r) = term_check(c) {
                    return r;
                }
            }
            _ => {}
        }
    }
    0
}
