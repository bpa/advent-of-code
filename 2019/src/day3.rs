use std::cmp::{Ord, Ordering};
use std::collections::{HashMap, HashSet};

#[aoc(day3, part1)]
pub fn closest_connection(input: &str) -> usize {
    let mut closest = Point(isize::max_value(), isize::max_value());
    let mut lines = input.split_whitespace();
    // We don't know what the grid size is yet, so it seems reasonable to store the known points
    let mut grid = HashSet::new();

    run_wire(lines.next().unwrap(), |point, _| {
        grid.insert(point);
    });
    run_wire(lines.next().unwrap(), |point, _| {
        if grid.contains(&point) && point < closest {
            closest = point;
        }
    });
    closest.manhatten_distance()
}

#[aoc(day3, part2)]
pub fn closest_connection_in_steps(input: &str) -> usize {
    let mut closest: usize = usize::max_value();
    let mut lines = input.split_whitespace();
    let mut grid = HashMap::new();

    run_wire(lines.next().unwrap(), |point, steps| {
        if let Some(existing) = grid.insert(point, steps) {
            // I don't really want to check for existance first, so I'll settle for if we replaced a value
            grid.insert(point, existing);
        }
    });
    run_wire(lines.next().unwrap(), |point, steps| {
        if let Some(first) = grid.get(&point) {
            let total = first + steps;
            if total < closest {
                closest = total;
            }
        }
    });
    closest
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd)]
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
    ($x:ident, $y:ident, $z:ident, $dist:ident, $f:tt, $axis:ident, $op:tt) => {
        for _ in 0..$dist {
            $axis = $axis $op 1;
            $z += 1;
            $f(Point($x, $y), $z);
        }
    };
}

fn run_wire<F>(path: &str, mut f: F)
where
    F: FnMut(Point, usize),
{
    let (mut x, mut y, mut z) = (0, 0, 0);

    for segment in path.split(",") {
        let (direction, distance_str) = segment.split_at(1);
        let distance = distance_str.parse::<usize>().unwrap();
        match direction {
            "U" => lay!(x, y, z, distance, f, y, +),
            "D" => lay!(x, y, z, distance, f, y, -),
            "L" => lay!(x, y, z, distance, f, x, -),
            "R" => lay!(x, y, z, distance, f, x, +),
            _ => panic!("Invalid direction given: {}", direction),
        };
    }
}
