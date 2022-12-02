#[derive(Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}
enum Strat {
    Lose,
    Draw,
    Win,
}
impl std::ops::Neg for Hand {
    type Output = i32;

    fn neg(self) -> Self::Output {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissor => 3,
        }
    }
}
impl std::ops::BitXor<Strat> for Hand {
    type Output = Self;
    fn bitxor(self, rhs: Strat) -> Self::Output {
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
}

#[derive(Clone, Copy)]
struct CChar(char);
impl std::ops::Neg for CChar {
    type Output = Hand;
    fn neg(self) -> Self::Output {
        let CChar(c) = self;
        match c {
            'A' => Hand::Rock,
            'B' => Hand::Paper,
            'C' => Hand::Scissor,
            'X' => Hand::Rock,
            'Y' => Hand::Paper,
            'Z' => Hand::Scissor,
            _ => unreachable!(),
        }
    }
}
impl std::ops::Not for CChar {
    type Output = Strat;
    fn not(self) -> Self::Output {
        let CChar(c) = self;
        match c {
            'X' => Strat::Lose,
            'Y' => Strat::Draw,
            'Z' => Strat::Win,
            _ => unreachable!(),
        }
    }
}

impl std::ops::Shr for Hand {
    type Output = i32;
    fn shr(self, rhs: Self) -> Self::Output {
        #![allow(clippy::suspicious_arithmetic_impl)]
        -rhs + match (self, rhs) {
            (Hand::Rock, Hand::Rock)
            | (Hand::Paper, Hand::Paper)
            | (Hand::Scissor, Hand::Scissor) => 3,
            (Hand::Rock, Hand::Paper)
            | (Hand::Paper, Hand::Scissor)
            | (Hand::Scissor, Hand::Rock) => 6,
            _ => 0,
        }
    }
}

fn str_to_choice(s: &str) -> (CChar, CChar) {
    match s.as_bytes() {
        [a, b' ', b, ..] => (CChar(*a as char), CChar(*b as char)),
        _ => unreachable!(),
    }
}

pub fn part1() -> i32 {
    include_str!("input/day2.input")
        .lines()
        .map(str_to_choice)
        .map(|(o, y)| -o >> -y)
        .sum()
}

pub fn part2() -> i32 {
    include_str!("input/day2.input")
        .lines()
        .map(str_to_choice)
        .map(|(o, y)| -o >> (-o ^ !y))
        .sum()
}
