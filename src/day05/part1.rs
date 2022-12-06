use std::borrow::BorrowMut;

use crate::day05::{Input, Output, Stacks};

pub fn solve(input: &Input) -> Output {
    let (stacks, operations) = input;
    let mut stacks = Stacks::new(stacks.clone()); // lame but CBA

    for operation in operations {
        stacks.perform_9000(*operation);
    }

    stacks.peek_tops().into_iter().collect::<String>().into()
}
