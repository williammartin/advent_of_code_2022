use crate::day02::Input;

use super::Round;

const INPUT: &str = include_str!("../../input/02/input.txt");

pub fn read() -> Input {
    INPUT.lines().map(parse_line).collect()
}

fn parse_line(s: &str) -> (String, String) {
    let mut components = s.split(" ");
    let first_char = components.next().unwrap();
    let second_char = components.next().unwrap();

    (first_char.to_owned(), second_char.to_owned())
}
