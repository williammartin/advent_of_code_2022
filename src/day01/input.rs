use crate::day01::Input;

const INPUT: &str = include_str!("../../input/01/input.txt");

pub fn read() -> Input {
    INPUT.split("\n\n").map(parse_elf_calories).collect()
}

fn parse_elf_calories(s: &str) -> u32 {
    s.lines().flat_map(|l| l.parse::<u32>()).sum()
}
