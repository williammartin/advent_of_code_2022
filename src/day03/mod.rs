pub mod input;
pub mod part1;
pub mod part2;

use std::{borrow::Borrow, marker::PhantomData, slice::Iter, str::FromStr};

use crate::{Output, Part};

pub type Compartment = Vec<char>;
pub struct Rucksack(Compartment, Compartment);

impl Rucksack {
    fn iter(&self) -> RucksackIter<'_, std::iter::Chain<Iter<char>, Iter<char>>> {
        let both_compartments_iter = self.0.iter().chain(self.1.iter());
        RucksackIter {
            iter: both_compartments_iter,
            phantom_data: &(),
        }
    }

    fn contains(&self, c: &char) -> bool {
        self.0.contains(c) || self.1.contains(c)
    }
}

struct RucksackIter<'a, I> {
    iter: I,
    phantom_data: &'a (),
}

impl<'a, I> Iterator for RucksackIter<'a, I>
where
    I: Iterator<Item = &'a char>,
{
    type Item = &'a char;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl FromIterator<char> for Rucksack {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        let mut v = vec![];
        for i in iter {
            v.push(i)
        }

        let second_comp = v.split_off(v.len() / 2);
        Rucksack(v, second_comp)
    }
}

pub type Input = Vec<Rucksack>;

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
