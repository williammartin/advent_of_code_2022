use crate::day04::Input;

use super::{AssignmentPair, Range};

const INPUT: &str = include_str!("../../input/04/input.txt");

pub fn read() -> Input {
    INPUT
        .lines()
        .flat_map(|line| line.parse::<AssignmentPair>()) // If a result was an error we'd be squashing it here
        .collect()
}
