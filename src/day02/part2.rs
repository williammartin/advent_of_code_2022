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
        "A" => Hand::Rock,
        "B" => Hand::Paper,
        "C" => Hand::Scissors,
        _ => panic!("unsupported character"),
    }
}

fn calculate_losing_hand(h: &Hand) -> Hand {
    match h {
        Hand::Rock => Hand::Scissors,
        Hand::Paper => Hand::Rock,
        Hand::Scissors => Hand::Paper,
    }
}

fn calculate_winning_hand(h: &Hand) -> Hand {
    match h {
        Hand::Rock => Hand::Paper,
        Hand::Paper => Hand::Scissors,
        Hand::Scissors => Hand::Rock,
    }
}

fn parse_round(line: &(String, String)) -> Round {
    let opponents_hand = parse_hand(line.0.as_str());
    let my_hand = match line.1.as_str() {
        "X" => calculate_losing_hand(&opponents_hand),
        "Y" => opponents_hand,
        "Z" => calculate_winning_hand(&opponents_hand),
        _ => panic!("unsupported character"),
    };

    Round {
        me: my_hand,
        opponent: opponents_hand,
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
