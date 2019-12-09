use std::cmp::{Ord, Ordering};
use std::collections::HashSet;

#[aoc(day3, part1)]
pub fn closest_connection(input: &str) -> usize {
    let mut closest = Point(isize::max_value(), isize::max_value());
    let mut lines = input.split_whitespace();
    // We don't know what the grid size is yet, so it seems reasonable to store the known points
    let mut grid = HashSet::new();

    run_wire(lines.next().unwrap(), |point| {
        grid.insert(point);
    });
    run_wire(lines.next().unwrap(), |point| {
        if grid.contains(&point) && point < closest {
            closest = point;
        }
    });
    closest.manhatten_distance()
}

#[derive(Hash, Eq, PartialEq, PartialOrd)]
struct Point(isize, isize);

impl Point {
    fn manhatten_distance(&self) -> usize {
        self.0.abs() as usize + self.1.abs() as usize
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.manhatten_distance().cmp(&other.manhatten_distance())
    }
}

// I tried assigning closures in the match, but I couln't get that to compile
// This is really just inlining a block to make the match in run_wire easy to read
macro_rules! lay {
    ($x:ident, $y:ident, $dist:ident, $f:tt, $axis:ident, $op:tt) => {
        for _ in 0..$dist {
            $axis = $axis $op 1;
            $f(Point($x, $y));
        }
    };
}

fn run_wire<F>(path: &str, mut f: F)
where
    F: FnMut(Point),
{
    let (mut x, mut y) = (0, 0);

    for segment in path.split(",") {
        let (direction, distance_str) = segment.split_at(1);
        let distance = distance_str.parse::<usize>().unwrap();
        match direction {
            "U" => lay!(x, y, distance, f, y, +),
            "D" => lay!(x, y, distance, f, y, -),
            "L" => lay!(x, y, distance, f, x, -),
            "R" => lay!(x, y, distance, f, x, +),
            _ => panic!("Invalid direction given: {}", direction),
        };
    }
}
