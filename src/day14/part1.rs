use crate::day14::{Input, Output};

use super::{Map, Point};

pub fn solve(points: &Input) -> Output {
    let mut map: Map = points.into_iter().cloned().collect(); // whatever, it's Advent of Code!

    let sand_starting_point = Point { x: 500, y: 0 }; // we can mutate this with the last free spot to avoid recalculating
    let mut resting_sand_counter = 0; // we could have the obstacle be a rock wall or sand enum and then count but that seems extra
    while let Some(p) = find_resting_point_for_sand(&map, sand_starting_point) {
        // while we have a resting point, insert it as an obstacle
        map.insert_sand_at(p);
        // and increment our counter
        resting_sand_counter = resting_sand_counter + 1;
    }

    resting_sand_counter.into()
}

fn find_resting_point_for_sand(map: &Map, starting_at: Point) -> Option<Point> {
    let mut current_position_of_sand = starting_at; // we're going to mutate this each time we find a valid position
    let mut at_rest = false; // we're going to set this to true once the sand has no where else to go

    while !at_rest {
        // If we're not at rest and our position is in the void, then we have no resting point
        if map.in_void(current_position_of_sand) {
            return None;
        }

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

    // Return the current position as the resting point
    Some(current_position_of_sand)
}
