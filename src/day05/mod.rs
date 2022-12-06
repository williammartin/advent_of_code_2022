pub mod input;
pub mod part1;
pub mod part2;

use std::vec::Drain;

use crate::{Output, Part};

type Stack = Vec<char>;

#[derive(Debug)]
pub struct Stacks(Vec<Stack>);

impl Stacks {
    fn new(stacks: Vec<Stack>) -> Stacks {
        Stacks(stacks)
    }

    fn push(&mut self, to: usize, c: char) {
        self.0[to - 1].push(c); // could panic of course but eh
    }

    fn pop(&mut self, from: usize, count: usize) -> Vec<char> {
        let len = self.0[from - 1].len();
        self.0[from - 1]
            .split_off(len - count)
            .into_iter()
            .rev()
            .collect()
    }

    pub fn perform_9000(&mut self, operation: Operation) {
        let chars = self.pop(operation.from, operation.count);
        for c in chars {
            self.push(operation.to, c);
        }
    }

    pub fn perform_9001(&mut self, operation: Operation) {
        let mut chars = self.pop(operation.from, operation.count);
        chars.reverse();
        for c in chars {
            self.push(operation.to, c);
        }
    }

    pub fn peek_tops(&self) -> Vec<char> {
        let mut tops = vec![];
        for stack in self.0.clone() {
            // omg lol terrible gotta get this done
            if let Some(c) = stack.last() {
                tops.push(c.clone());
            }
        }

        tops
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Operation {
    count: usize, // too much space but eh
    from: usize,
    to: usize,
}

pub type Input = (Vec<Stack>, Vec<Operation>);

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
