pub fn part1() -> i32 {
    let mut input = include_str!("day2.input")
        .trim()
        .split(',')
        .map(|s| s.parse::<_>().unwrap())
        .collect::<Vec<_>>();
    input[1] = 12;
    input[2] = 2;
    let mut i = 0;
    loop {
        match input[i] {
            1 => {
                let pos = input[i + 3] as usize;
                input[pos] = input[input[i + 1] as usize] + input[input[i + 2] as usize]
            }
            2 => {
                let pos = input[i + 3] as usize;
                input[pos] = input[input[i + 1] as usize] * input[input[i + 2] as usize]
            }
            99 => break,
            _ => panic!(),
        };
        i += 4;
    }
    input[0]
}

pub fn part2() -> i32 {
    let input = include_str!("day2.input")
        .trim()
        .split(',')
        .map(|s| s.parse::<_>().unwrap())
        .collect::<Vec<_>>();
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut input = input.clone();
            input[1] = noun;
            input[2] = verb;
            let mut i = 0;
            loop {
                match input[i] {
                    1 => {
                        let pos = input[i + 3] as usize;
                        input[pos] = input[input[i + 1] as usize] + input[input[i + 2] as usize]
                    }
                    2 => {
                        let pos = input[i + 3] as usize;
                        input[pos] = input[input[i + 1] as usize] * input[input[i + 2] as usize]
                    }
                    99 => break,
                    _ => panic!(),
                };
                i += 4;
            }
            if input[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    0
}
