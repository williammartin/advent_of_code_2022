use std::ops::Add;

use crate::day08::{Input, Output};

pub fn solve(map: &Input) -> Output {
    map.trees() // get all trees
        .filter(|tree| map.is_tree_interior(*tree)) // include only those that are interior trees
        .filter(|tree| {
            // include trees where any line of trees has visibility
            let mut tree_lines = map.trees_in_line_of_sight(*tree);
            // Are there any tree lines where all the trees are lower than our current tree?
            // If yes, then at least one line has visibility.
            tree_lines.any(|tree_line| {
                tree_line
                    .iter()
                    .all(|los_tree| los_tree.height < tree.height)
            })
        })
        .count()
        .add(map.border_size())
        .into()
}
