use crate::day08::{Input, Output};

pub fn solve(map: &Input) -> Output {
    map.trees() // get all trees
        .filter(|tree| map.is_tree_interior(*tree)) // include only those that are interior trees
        .map(|tree| {
            // include trees where any line of trees has visibility
            let mut tree_lines = map.trees_in_line_of_sight(tree);
            // Take from each tree line until we find a tree the same height or taller than our current tree
            // Then fold each tree line multiplying together their lengths
            tree_lines
                .map(|tree_line| {
                    let mut lower_trees_in_line = vec![];
                    for los_tree in tree_line {
                        if los_tree.height < tree.height {
                            // if the tree in the line is lower then append
                            lower_trees_in_line.push(los_tree);
                        } else {
                            // otherwise append and break the loop because we shouldn't include any past this
                            // Super lame. Really I want something like take_while that is inclusive.
                            lower_trees_in_line.push(los_tree);
                            break;
                        }
                    }

                    lower_trees_in_line
                })
                .fold(1, |acc, lower_trees_in_line| {
                    acc * lower_trees_in_line.len()
                })
        })
        .max()
        .expect("to have at least one tree with a scenic score")
        .into()
}
