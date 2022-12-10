enum Instruction {
    NoOp,
    AddX(i32),
}

pub fn part1() -> i32 {
    let mut x = 1;
    let mut cycle = 0;
    let mut signal_strenght = 0;
    include_str!("input/day10.input")
        .lines()
        .map(|s| {
            let mut splat = s.split_whitespace().skip(1);
            if let Some(a) = splat.next() {
                Instruction::AddX(a.parse().unwrap())
            } else {
                Instruction::NoOp
            }
        })
        .for_each(|i| {
            let mut adder = 0;
            let wait;
            match i {
                Instruction::NoOp => wait = 1,
                Instruction::AddX(o) => {
                    adder = o;
                    wait = 2;
                }
            };
            for i in 0..wait {
                cycle += 1;
                if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                    signal_strenght += x * cycle;
                }
                if i == wait - 1 {
                    x += adder;
                }
            }
        });
    signal_strenght
}

pub fn part2() -> &'static str {
    let mut x = 1;
    let mut cycle = 0;
    let mut canvas = vec![vec!["-"; 40]; 6];
    include_str!("input/day10.input")
        .lines()
        .map(|s| {
            let mut splat = s.split_whitespace().skip(1);
            if let Some(a) = splat.next() {
                Instruction::AddX(a.parse().unwrap())
            } else {
                Instruction::NoOp
            }
        })
        .for_each(|i| {
            let mut adder = 0;
            let wait;
            match i {
                Instruction::NoOp => wait = 1,
                Instruction::AddX(o) => {
                    adder = o;
                    wait = 2;
                }
            };
            for i in 0..wait {
                let index_x = cycle % 40;
                let index_y = cycle / 40;
                cycle += 1;
                canvas[index_y][index_x] = if index_x as i32 >= x - 1 && index_x as i32 <= x + 1 {
                    "#"
                } else {
                    "."
                };

                if i == wait - 1 {
                    x += adder;
                }
            }
        });
    canvas.into_iter().for_each(|r| {
        println!("{}", r.join(""));
    });
    "FECZELHE (question returns ascii art)"
}
