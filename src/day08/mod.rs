pub mod input;
pub mod part1;
pub mod part2;

use crate::{Output, Part};

#[derive(Debug, Clone, Copy)]
pub struct Tree {
    height: u8,
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct Map(Vec<Vec<u8>>);

impl Map {
    fn trees(&self) -> impl Iterator<Item = Tree> + '_ {
        self.0
            .iter()
            .enumerate()
            .map(move |(y, row)| {
                row.iter().enumerate().map(move |(x, height)| Tree {
                    height: *height,
                    x,
                    y,
                })
            })
            .flatten()
    }

    fn is_tree_interior(&self, tree: Tree) -> bool {
        !(tree.x == 0 || tree.x == self.0[0].len() - 1 || tree.y == 0 || tree.y == self.0.len() - 1)
        // errrrrrr... maybe? Not so good at bounds checking
    }

    fn trees_in_line_of_sight(&self, tree: Tree) -> impl Iterator<Item = Vec<Tree>> + '_ {
        let mut north = Vec::new();
        let mut south = Vec::new();
        let mut east = Vec::new();
        let mut west = Vec::new();

        for los_tree in self.trees() {
            match direction(tree, los_tree) {
                Direction::None | Direction::Diagonal => {} // remove the current tree and any diagonals
                Direction::North => north.push(los_tree),
                Direction::South => south.push(los_tree),
                Direction::East => east.push(los_tree),
                Direction::West => west.push(los_tree),
            };
        }

        // reverse north and west so that the coords always go from the Tree outwards.
        north.reverse();
        west.reverse();

        vec![north, south, east, west].into_iter()
    }

    fn border_size(&self) -> usize {
        // lol calculate top and bottom and then remove corners and calclate left and right
        (self.0.len() + self.0[0].len()) * 2 - 4
    }
}

enum Direction {
    None,
    North,
    South,
    East,
    West,
    Diagonal,
}

fn direction(from: Tree, to: Tree) -> Direction {
    // probably can use trig for this but maths sux
    if from.x == to.x && from.y == to.y {
        // same point
        Direction::None
    } else {
        if from.x == to.x {
            // we're in the same column
            if from.y < to.y {
                Direction::South
            } else {
                Direction::North
            }
        } else if from.y == to.y {
            // we're in the same row
            if from.x < to.x {
                Direction::East
            } else {
                Direction::West
            }
        } else {
            // If we're not in the same row or colunn we must be on a diagonal
            Direction::Diagonal
        }
    }
}

impl FromIterator<Vec<u8>> for Map {
    fn from_iter<T: IntoIterator<Item = Vec<u8>>>(iter: T) -> Self {
        let mut m = Vec::new();

        for i in iter {
            m.push(i);
        }

        Map(m)
    }
}

pub type Input = Map;

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
