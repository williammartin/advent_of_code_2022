use std::cmp::{max, min};

use crate::day04::{Input, Output};

pub fn solve(input: &Input) -> Output {
    input
        .iter()
        .fold(0, |acc, pair| {
            if pair.0.overlaps(&pair.1) {
                acc + 1
            } else {
                acc
            }
        })
        .into()
}
