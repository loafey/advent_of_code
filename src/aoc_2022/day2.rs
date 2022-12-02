#[derive(Debug, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}
impl Hand {
    fn value(self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissor => 3,
        }
    }
    fn game(self, me: Self) -> i32 {
        me.value()
            + match (self, me) {
                (Hand::Rock, Hand::Rock)
                | (Hand::Paper, Hand::Paper)
                | (Hand::Scissor, Hand::Scissor) => 3,
                (Hand::Rock, Hand::Paper)
                | (Hand::Paper, Hand::Scissor)
                | (Hand::Scissor, Hand::Rock) => 6,
                _ => 0,
            }
    }
    fn enter_gamer_mode(self, rhs: Strat) -> Self {
        match (self, rhs) {
            (Hand::Rock, Strat::Win) => Hand::Paper,
            (Hand::Rock, Strat::Lose) => Hand::Scissor,
            (Hand::Paper, Strat::Win) => Hand::Scissor,
            (Hand::Paper, Strat::Lose) => Hand::Rock,
            (Hand::Scissor, Strat::Win) => Hand::Rock,
            (Hand::Scissor, Strat::Lose) => Hand::Paper,
            _ => self,
        }
    }
    fn from_char(c: char) -> Self {
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
}

enum Strat {
    Win,
    Lose,
    Draw,
}
impl Strat {
    fn from_char(c: char) -> Strat {
        match c {
            'Y' => Strat::Draw,
            'X' => Strat::Lose,
            'Z' => Strat::Win,
            _ => unreachable!(),
        }
    }
}

fn str_to_choice(s: &str) -> (char, char) {
    let mut split = s.chars();
    let first = split.next().unwrap();
    split.next();
    let second = split.next().unwrap();

    (first, second)
}

pub fn part1() -> i32 {
    include_str!("input/day2.input")
        .lines()
        .map(str_to_choice)
        .map(|(a, b)| (Hand::from_char(a), Hand::from_char(b)))
        .map(|(o, y)| o.game(y))
        .sum()
}

pub fn part2() -> i32 {
    include_str!("input/day2.input")
        .lines()
        .map(str_to_choice)
        .map(|(a, b)| (Hand::from_char(a), Strat::from_char(b)))
        .map(|(o, s)| (o, o.enter_gamer_mode(s)))
        .map(|(o, y)| o.game(y))
        .sum()
}
