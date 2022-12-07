use crate::day07::Input;

use super::{File, Filesystem, Line};

const INPUT: &str = include_str!("../../input/07/input.txt");

pub fn read() -> Input {
    INPUT.lines().map(parse_line).collect::<Filesystem>()
}

fn parse_line(s: &str) -> Line {
    // could flat_map at the call site but I guess I'd rather it blew up here
    s.parse().expect("to parse lines correctly")
}
