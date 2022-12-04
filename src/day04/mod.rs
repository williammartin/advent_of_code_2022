pub mod input;
pub mod part1;
pub mod part2;

use std::{
    cmp::{max, min},
    str::FromStr,
};

use crate::{Output, Part};

pub struct Range {
    lower: u32,
    upper: u32,
}

impl FromStr for Range {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bounds = s.split_once("-").ok_or("no hyphen found")?;
        let lower_bound = bounds
            .0
            .parse::<u32>()
            .map_err(|_| "couldn't parse lower bound")?;
        let upper_bound = bounds
            .1
            .parse::<u32>()
            .map_err(|_| "couldn't parse upper bound")?;

        Ok(Range {
            lower: lower_bound,
            upper: upper_bound,
        })
    }
}

impl Range {
    fn contains(&self, r: &Range) -> bool {
        self.lower <= r.lower && self.upper >= r.upper
    }

    fn overlaps(&self, r: &Range) -> bool {
        max(self.lower, r.lower) <= min(self.upper, r.upper)
    }
}

pub struct AssignmentPair(Range, Range);

impl FromStr for AssignmentPair {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pairs = s.split_once(",").ok_or("no comma found")?;
        let first_range = pairs.0.parse::<Range>()?;
        let second_range = pairs.1.parse::<Range>()?;

        Ok(AssignmentPair(first_range, second_range))
    }
}

pub type Input = Vec<AssignmentPair>;

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
