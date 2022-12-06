use crate::day05::Input;

use super::Operation;

const INPUT: &str = include_str!("../../input/05/input.txt");

fn parse_operation(input: &str) -> Operation {
    let mut parts = input.split_whitespace();

    // Skip the "move" keyword
    parts.next();
    // Lol ChatGPT
    let count = parts.next().unwrap().parse().unwrap();
    let from = parts.nth(1).unwrap().parse().unwrap();
    let to = parts.nth(1).unwrap().parse().unwrap();

    Operation { count, from, to }
}

pub fn read() -> Input {
    let (stacks_str, operations_str) = INPUT
        .split_once("\n\n")
        .expect("to split the stacks from the operations");

    let mut stacks = Vec::new();
    // Populate with 9 empty stacks of chars
    for _ in 0..9 {
        stacks.push(Vec::new());
    }

    for line in stacks_str.lines() {
        // Stop when line starts with " 1"
        if line.starts_with(" 1") {
            break;
        }
        // Add one space to the end of the line
        let line = format!("{} ", line);
        // Read it four characters at a time
        let mut stack_index = 0;
        for chunk in line.as_bytes().chunks(4) {
            // Look at second character in chunk
            let c = chunk[1] as char;
            if c != ' ' {
                stacks[stack_index].push(c);
            }
            stack_index += 1;
        }
    }

    let operations: Vec<Operation> = operations_str.lines().map(parse_operation).collect();

    for stack in stacks.iter_mut() {
        stack.reverse();
    }
    (stacks, operations)
}
