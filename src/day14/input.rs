use std::{collections::HashSet, str::FromStr};

use crate::day14::Input;

use super::{Line, Map, Obstacle, Point};

const INPUT: &str = include_str!("../../input/14/input.txt");

pub fn read() -> Input {
    let rock_lines = INPUT
        .lines() // get each line
        .map(|line| line.split(" -> ").flat_map(Point::from_str).collect()) // split each line into points that represent corners
        .collect::<Vec<Vec<Point>>>();

    let mut points = Vec::new();

    for line in rock_lines {
        // for every line of rocks
        let corner_pairs = line.windows(2); // get a pair that represent the ends of a line

        for pair in corner_pairs {
            // for each pair,
            // if we have two points (simple way to destructure the points though I think windows might handle this for us, this should make it panic safe)
            if let [Point { .. }, Point { .. }] = pair {
                // create a line from our two points
                let line: Line = (pair[0], pair[1]).into();
                // and for every point in that line, insert it into our map
                for point in line.points() {
                    points.push(point);
                }
            }
        }
    }

    points
}
