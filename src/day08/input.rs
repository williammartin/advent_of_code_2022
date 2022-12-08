use crate::day08::Input;

const INPUT: &str = include_str!("../../input/08/input.txt");

pub fn read() -> Input {
    INPUT.lines().map(parse_line).collect()
}

fn parse_line(s: &str) -> Vec<u8> {
    s.split("").flat_map(|c| c.parse::<u8>()).collect()
}
