use crate::day03::{Input, Output};

use super::Rucksack;

pub fn solve(input: &Input) -> Output {
    input
        .iter()
        .map(find_duplicate_item)
        .map(convert_to_value)
        .sum::<u32>()
        .into()
}

fn find_duplicate_item(r: &Rucksack) -> char {
    let mut first_compartment_iter = r.0.iter();
    loop {
        let elt = first_compartment_iter.next();
        if let Some(i) = elt {
            if r.1.contains(i) {
                return i.clone();
            }
        } else {
            panic!("ran out of items before finding a duplicate...replace with Option")
        }
    }
}

fn convert_to_value(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - ('a' as u32) + 1,
        'A'..='Z' => (c as u32) - ('A' as u32) + 27,
        _ => panic!("can't convert {} to priority", c),
    }
}
