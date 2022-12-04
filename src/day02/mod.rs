pub mod input;
pub mod part1;
pub mod part2;

use crate::{Output, Part};

#[derive(Clone, Copy)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

pub struct Round {
    me: Hand,
    opponent: Hand,
}

pub enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Round {
    pub fn determine_outcome(&self) -> Outcome {
        match (self.me, self.opponent) {
            (Hand::Rock, Hand::Paper)
            | (Hand::Paper, Hand::Scissors)
            | (Hand::Scissors, Hand::Rock) => Outcome::Loss,
            (Hand::Rock, Hand::Scissors)
            | (Hand::Paper, Hand::Rock)
            | (Hand::Scissors, Hand::Paper) => Outcome::Win,
            _ => Outcome::Draw,
        }
    }
}

pub type Input = Vec<(String, String)>;

pub fn run(part: Part) -> Output {
    let input = input::read();
    match part {
        Part::One => part1::solve(&input),
        Part::Two => part2::solve(&input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_answer_one() {
        let result = run(Part::One);
        println!("{result}");
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::Two);
        println!("{result}");
    }
}
