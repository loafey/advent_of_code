#[derive(Debug, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}
fn char_to_hand(c: char) -> Hand {
    match c {
        'A' => Hand::Rock,
        'B' => Hand::Paper,
        'C' => Hand::Scissor,
        'Y' => Hand::Paper,
        'X' => Hand::Rock,
        'Z' => Hand::Scissor,
        _ => unreachable!(),
    }
}
enum Strat {
    Win,
    Lose,
    Draw,
}
fn char_to_strat(c: char) -> Strat {
    match c {
        'Y' => Strat::Draw,
        'X' => Strat::Lose,
        'Z' => Strat::Win,
        _ => unreachable!(),
    }
}

pub fn part1() -> i32 {
    include_str!("input/day2.input")
        .lines()
        .map(|r| {
            let mut split = r.chars();
            let first = split.next().unwrap();
            split.next();
            let second = split.next().unwrap();

            (char_to_hand(first), char_to_hand(second))
        })
        .map(|(o, y)| match (o, y) {
            (Hand::Rock, Hand::Rock) => 1 + 3,
            (Hand::Rock, Hand::Paper) => 2 + 6,
            (Hand::Rock, Hand::Scissor) => 3 + 0,
            (Hand::Paper, Hand::Rock) => 1 + 0,
            (Hand::Paper, Hand::Paper) => 2 + 3,
            (Hand::Paper, Hand::Scissor) => 3 + 6,
            (Hand::Scissor, Hand::Rock) => 1 + 6,
            (Hand::Scissor, Hand::Paper) => 2 + 0,
            (Hand::Scissor, Hand::Scissor) => 3 + 3,
        })
        .sum()
}

pub fn part2() -> i32 {
    include_str!("input/day2.input")
        .lines()
        .map(|r| {
            let mut split = r.chars();
            let first = split.next().unwrap();
            split.next();
            let second = split.next().unwrap();

            (char_to_hand(first), char_to_strat(second))
        })
        .map(|(o, s)| {
            (
                o,
                match (o, s) {
                    (Hand::Rock, Strat::Win) => Hand::Paper,
                    (Hand::Rock, Strat::Lose) => Hand::Scissor,
                    (Hand::Rock, Strat::Draw) => Hand::Rock,
                    (Hand::Paper, Strat::Win) => Hand::Scissor,
                    (Hand::Paper, Strat::Lose) => Hand::Rock,
                    (Hand::Paper, Strat::Draw) => Hand::Paper,
                    (Hand::Scissor, Strat::Win) => Hand::Rock,
                    (Hand::Scissor, Strat::Lose) => Hand::Paper,
                    (Hand::Scissor, Strat::Draw) => Hand::Scissor,
                },
            )
        })
        .map(|(o, y)| match (o, y) {
            (Hand::Rock, Hand::Rock) => 1 + 3,
            (Hand::Rock, Hand::Paper) => 2 + 6,
            (Hand::Rock, Hand::Scissor) => 3 + 0,
            (Hand::Paper, Hand::Rock) => 1 + 0,
            (Hand::Paper, Hand::Paper) => 2 + 3,
            (Hand::Paper, Hand::Scissor) => 3 + 6,
            (Hand::Scissor, Hand::Rock) => 1 + 6,
            (Hand::Scissor, Hand::Paper) => 2 + 0,
            (Hand::Scissor, Hand::Scissor) => 3 + 3,
        })
        .sum()
}
