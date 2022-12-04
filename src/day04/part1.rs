use crate::day04::{Input, Output};

use super::{AssignmentPair, Range};

pub fn solve(input: &Input) -> Output {
    input
        .iter()
        .fold(0, |acc, pair| {
            if pair.0.contains(&pair.1) || pair.1.contains(&pair.0) {
                acc + 1
            } else {
                acc
            }
        })
        .into()
}
