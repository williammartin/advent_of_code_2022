pub mod input;
pub mod part1;
pub mod part2;

use std::{
    cmp::{max, min},
    collections::HashSet,
    fmt::Display,
    str::FromStr,
};

use crate::{Output, Part};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: u16,
    y: u16,
}

impl FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(",")
            .ok_or("unable to split coordinate")
            .and_then(|(x, y)| {
                let x = x.parse::<u16>().map_err(|_| "unable to parse x")?;
                let y = y.parse::<u16>().map_err(|_| "unable to parse y")?;

                Ok(Point { x, y })
            })
    }
}

pub struct Line {
    start: Point,
    end: Point,
}

impl From<(Point, Point)> for Line {
    fn from(points: (Point, Point)) -> Self {
        Line {
            start: points.0,
            end: points.1,
        }
    }
}

impl Line {
    fn points(&self) -> impl Iterator<Item = Point> + '_ {
        let mut points = Vec::new();

        if self.start.x == self.end.x {
            // if x coords are the same we are dealing with a vertical line (or single point)
            // get the ends of the line on the vertical axis
            let mut start_y = min(self.start.y, self.end.y);
            let end_y = max(self.start.y, self.end.y);

            // insert each point counting up between the two
            while start_y <= end_y {
                points.push(Point {
                    x: self.start.x,
                    y: start_y,
                });

                start_y = start_y + 1;
            }
        } else if self.start.y == self.end.y {
            // otherwise we are dealing with a horizontal line (or single point)
            let mut start_x = min(self.start.x, self.end.x);
            let end_x = max(self.start.x, self.end.x);

            while start_x <= end_x {
                points.push(Point {
                    x: start_x,
                    y: self.start.y,
                });

                start_x = start_x + 1;
            }
        } else {
            panic!("unexpected diagonal line")
        }

        points.into_iter()
    }
}

pub type Obstacle = Point;

#[derive(Debug)]
pub struct Map {
    obstacles: HashSet<Obstacle>,
    void_begins_at: u16,
}

impl Map {
    fn new() -> Map {
        Map {
            obstacles: HashSet::new(),
            void_begins_at: 0,
        }
    }

    fn insert_obstacle_at(&mut self, o: Obstacle) {
        self.obstacles.insert(o);
    }

    fn has_space_at(&self, p: Point) -> bool {
        !self.obstacles.contains(&p)
    }

    fn in_void(&self, p: Point) -> bool {
        // If we're greater, we're obviously in the void and if we're equal then that means
        // we must have fallen to the left or right but either way we're falling into the void.
        p.y >= self.void_begins_at
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = self
            .obstacles
            .iter()
            .map(|o| o.x)
            .min()
            .expect("to have at least one point");
        let min_y = self
            .obstacles
            .iter()
            .map(|o| o.y)
            .min()
            .expect("to have at least one point");

        let max_x = self
            .obstacles
            .iter()
            .map(|o| o.x)
            .max()
            .expect("to have at least one point");
        let max_y = self
            .obstacles
            .iter()
            .map(|o| o.y)
            .max()
            .expect("to have at least one point");

        // for each line
        for y in min_y..=max_y {
            // for each cell
            for x in min_x..=max_x {
                if self.has_space_at(Point { x, y }) {
                    write!(f, ".")?
                } else {
                    write!(f, "#")?
                }
            }
            writeln!(f)?
        }

        Ok(())
    }
}

impl FromIterator<Point> for Map {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        let mut map = Map::new();

        for p in iter {
            map.insert_obstacle_at(p);
            if p.y > map.void_begins_at {
                map.void_begins_at = p.y;
            }
        }

        map
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct FlooredMap {
    map: Map,
    floor_y: u16,
}

impl FlooredMap {
    fn new(map: Map) -> FlooredMap {
        let floor_y = map.void_begins_at + 2;

        FlooredMap { map, floor_y }
    }

    fn insert_obstacle_at(&mut self, o: Obstacle) {
        self.map.insert_obstacle_at(o);
    }

    fn has_space_at(&self, p: Point) -> bool {
        // As this is a floored map, we need to assume there is no space if we
        // reach max y of our current obstacles + 2.

        // look at all the points, map them to their y coord, and choose the largest.
        // this indicates where the bottom of our map is, so add 2 to predict where the floor is

        // if we have equality, then this point is _on_ the floor
        if p.y == self.floor_y {
            false
        } else {
            self.map.has_space_at(p)
        }
    }
}

impl Display for FlooredMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.map);

        let min_x = self
            .map
            .obstacles
            .iter()
            .map(|o| o.x)
            .min()
            .expect("to have at least one point");

        let max_x = self
            .map
            .obstacles
            .iter()
            .map(|o| o.x)
            .max()
            .expect("to have at least one point");

        // create new points for our predicted lines
        let predicted_lines = vec![
            Line {
                start: Point {
                    x: min_x,
                    y: self.floor_y - 1,
                },
                end: Point {
                    x: max_x,
                    y: self.floor_y - 1,
                },
            },
            Line {
                start: Point {
                    x: min_x,
                    y: self.floor_y,
                },
                end: Point {
                    x: max_x,
                    y: self.floor_y,
                },
            },
        ];

        for line in predicted_lines {
            for point in line.points() {
                if self.has_space_at(point) {
                    write!(f, ".")?
                } else {
                    write!(f, "#")?
                }
            }
            writeln!(f)?
        }

        Ok(())
    }
}

pub type Input = Vec<Point>;

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
