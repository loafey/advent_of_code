use crate::utils::ascii_4_art_to_string;

enum Instruction {
    NoOp,
    AddX(i32),
}
impl From<Instruction> for (i32, i32) {
    fn from(val: Instruction) -> (i32, i32) {
        match val {
            Instruction::NoOp => (0, 1),
            Instruction::AddX(o) => (o, 2),
        }
    }
}

fn parse_ops() -> impl Iterator<Item = Instruction> {
    include_str!("input/day10.input").lines().map(|s| {
        let mut splat = s.split_whitespace().skip(1);
        splat
            .next()
            .map(|a| Instruction::AddX(a.parse().unwrap()))
            .unwrap_or(Instruction::NoOp)
    })
}

pub fn part1() -> i32 {
    let mut x = 1;
    let mut cycle = 0;
    let mut signal_strenght = 0;
    parse_ops().for_each(|i| {
        let (adder, wait) = i.into();
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

pub fn part2() -> String {
    let mut x = 1;
    let mut cycle = 0;
    let mut canvas = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];
    parse_ops().for_each(|i| {
        let (adder, wait) = i.into();
        for i in 0..wait {
            let index_x = cycle % 40;
            let index_y = cycle / 40;
            cycle += 1;
            canvas[index_y].push(if index_x as i32 >= x - 1 && index_x as i32 <= x + 1 {
                '#'
            } else {
                '.'
            });

            if i == wait - 1 {
                x += adder;
            }
        }
    });
    ascii_4_art_to_string(&canvas, 1)
}
