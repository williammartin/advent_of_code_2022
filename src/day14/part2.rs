use crate::day14::{Input, Output};

use super::{FlooredMap, Map, Point};

pub fn solve(points: &Input) -> Output {
    let mut map = FlooredMap::new(points.into_iter().cloned().collect()); // whatever, it's Advent of Code!

    let sand_starting_point = Point { x: 500, y: 0 }; // we can mutate this with the last free spot to avoid recalculating
    let mut resting_sand_counter = 0; // we could have the obstacle be a rock wall or sand enum and then count but that seems extra
    while let Some(p) = find_resting_point_for_sand(&map, sand_starting_point) {
        // while we have a resting point, insert it as an obstacle
        map.insert_obstacle_at(p);
        // and increment our counter
        resting_sand_counter = resting_sand_counter + 1;
    }

    (resting_sand_counter + 1).into() // +1 because we need to count the last placed sand
}

fn find_resting_point_for_sand(map: &FlooredMap, starting_at: Point) -> Option<Point> {
    let mut current_position_of_sand = starting_at; // we're going to mutate this each time we find a valid position
    let mut at_rest = false; // we're going to set this to true once the sand has no where else to go

    while !at_rest {
        let (x, y) = (current_position_of_sand.x, current_position_of_sand.y);

        // Try down
        let attempted_point = Point { x, y: y + 1 };
        if map.has_space_at(attempted_point) {
            current_position_of_sand = attempted_point;
            continue;
        }

        // then down left
        let attempted_point = Point { x: x - 1, y: y + 1 };
        if map.has_space_at(attempted_point) {
            current_position_of_sand = attempted_point;
            continue;
        }

        // then down right
        let attempted_point = Point { x: x + 1, y: y + 1 };
        if map.has_space_at(attempted_point) {
            current_position_of_sand = attempted_point;
            continue;
        }

        // otherwise we're at rest
        at_rest = true;
    }

    // If our position hasn't changed that means there is no space
    if current_position_of_sand == starting_at {
        None
    } else {
        // Return the current position as the resting point
        Some(current_position_of_sand)
    }
}
