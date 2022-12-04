use crate::day04::Input;

use super::{AssignmentPair, Range};

const INPUT: &str = include_str!("../../input/04/input.txt");

pub fn read() -> Input {
    INPUT
        .lines()
        .map(|line| {
            line.parse::<AssignmentPair>()
                .expect("to parse assignment pair correctly")
        })
        .collect()
}
