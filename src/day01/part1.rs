use crate::day01::{Input, Output};

pub fn solve(input: &Input) -> Output {
    input.iter().max().unwrap().to_owned().into()
}
