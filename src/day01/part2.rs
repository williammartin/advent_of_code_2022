use crate::day01::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut owned_input = input.to_owned();
    owned_input.sort();
    owned_input.iter().rev().take(3).sum::<u32>().into()
}
