pub fn part1() -> i32 {
    let input = 18;
    let cells = (1..=300)
        .map(|y| {
            (1..=300)
                .map(|x| {
                    let mut power_level = x + 10;
                    power_level *= y;
                    power_level += input;
                    power_level *= x + 10;
                    power_level = format!("{power_level}")
                        .chars()
                        .rev()
                        .nth(2)
                        .unwrap()
                        .to_string()
                        .parse()
                        .unwrap();
                    power_level -= 5;

                    power_level
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    println!("{}", cells[5 - 1][3 - 1]);

    &cells[44..44 + 5].iter().for_each(|r| {
        r[32..32 + 5].iter().for_each(|i| print!("{i}\t"));
        println!()
    });

    0
}

pub fn part2() -> i32 {
    0
}
