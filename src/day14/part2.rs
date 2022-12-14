use crate::day14::{Input, Output};

use super::{FlooredMap, Map, Point};

pub fn solve(points: &Input) -> Output {
    let mut map = FlooredMap::new(points.into_iter().cloned().collect()); // whatever, it's Advent of Code!

    let mut path_taken = vec![Point { x: 500, y: 0 }];
    let mut resting_sand_counter = 0; // we could have the obstacle be a rock wall or sand enum and then count but that seems extra
    while let Some(mut path) = find_resting_point_for_sand(&map, path_taken.clone()) {
        // while we have a resting point, insert it as an obstacle
        map.insert_sand_at(path.pop().expect("to have an obstacle"));
        // and increment our counter
        resting_sand_counter = resting_sand_counter + 1;
        path_taken = path;
    }

    resting_sand_counter.into() // +1 because we need to count the last placed sand
}

fn find_resting_point_for_sand(
    map: &FlooredMap,
    mut path_taken: Vec<Point>, // super stupid
) -> Option<Vec<Point>> {
    // Oh my god this is ridiculous but I spent too much time on it and it works so fuuuuuuu...
    while let Some(starting_point) = path_taken.pop() {
        if !map.has_space_at(starting_point) {
            continue;
        }

        let mut current_position_of_sand = starting_point; // we're going to mutate this each time we find a valid position
        let mut at_rest = false; // we're going to set this to true once the sand has no where else to go

        while !at_rest {
            let (x, y) = (current_position_of_sand.x, current_position_of_sand.y);

            // Try down
            let attempted_point = Point { x, y: y + 1 };
            if map.has_space_at(attempted_point) {
                path_taken.push(current_position_of_sand);
                current_position_of_sand = attempted_point;
                continue;
            }

            // then down left
            let attempted_point = Point { x: x - 1, y: y + 1 };
            if map.has_space_at(attempted_point) {
                path_taken.push(current_position_of_sand);
                current_position_of_sand = attempted_point;
                continue;
            }

            // then down right
            let attempted_point = Point { x: x + 1, y: y + 1 };
            if map.has_space_at(attempted_point) {
                path_taken.push(current_position_of_sand);
                current_position_of_sand = attempted_point;
                continue;
            }

            // otherwise we're at rest
            at_rest = true;
        }

        // Return the current position as the resting point
        path_taken.push(current_position_of_sand);
        return Some(path_taken);
    }

    return None;
}
