use crate::day03::Input;

use super::Rucksack;

const INPUT: &str = include_str!("../../input/03/input.txt");

pub fn read() -> Input {
    INPUT.lines().map(parse_rucksack).collect()
}

fn parse_rucksack(s: &str) -> Rucksack {
    s.chars().collect()
}
