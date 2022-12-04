use super::*;
use crate::day02::{Input, Output};

pub fn solve(input: &Input) -> Output {
    input
        .iter()
        .map(parse_round)
        .map(calculate_score)
        .sum::<u32>()
        .into()
}

fn parse_hand(s: &str) -> Hand {
    match s {
        "A" | "X" => Hand::Rock,
        "B" | "Y" => Hand::Paper,
        "C" | "Z" => Hand::Scissors,
        _ => panic!("unsupported character"),
    }
}

fn parse_round(line: &(String, String)) -> Round {
    Round {
        me: parse_hand(line.1.as_str()),
        opponent: parse_hand(line.0.as_str()),
    }
}

fn calculate_score(r: Round) -> u32 {
    let hand_score = match r.me {
        super::Hand::Rock => 1,
        super::Hand::Paper => 2,
        super::Hand::Scissors => 3,
    };

    let outcome_score = match r.determine_outcome() {
        super::Outcome::Win => 6,
        super::Outcome::Draw => 3,
        super::Outcome::Loss => 0,
    };

    hand_score + outcome_score
}
