use crate::day07::Input;

use super::{File, Line};

const INPUT: &str = include_str!("../../input/07/input.txt");

pub fn read() -> Input {
    // let root = File::Directory {
    //     name: "/".to_owned(),
    //     files: vec![],
    // };
    // let mut cd = root;

    let lines: Vec<Line> = INPUT.lines().map(parse_line).collect();
    dbg!(lines);

    for line in INPUT.lines() {}

    todo!();
}

fn parse_line(s: &str) -> Line {
    // could flat_map at the call site but I guess I'd rather it blew up here
    s.parse().expect("to parse lines correctly")
}
