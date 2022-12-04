use crate::day03::{Input, Output};

use super::Rucksack;

pub fn solve(input: &Input) -> Output {
    input
        .chunks(3)
        .map(find_badge_char_in_3_rucks)
        .map(convert_to_value)
        .sum::<u32>()
        .into()
}

fn find_badge_char_in_3_rucks(rs: &[Rucksack]) -> char {
    let mut first_ruck_iter = rs[0].iter();
    loop {
        let elt = first_ruck_iter.next();
        if let Some(i) = elt {
            if rs[1].contains(i) && rs[2].contains(i) {
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
