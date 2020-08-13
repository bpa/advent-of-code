use std::cmp;
use std::convert::TryFrom;
use std::default::Default;
use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
struct AxialCoordinate {
    q: isize,
    r: isize,
}

impl AxialCoordinate {
    fn distance(self, other: Self) -> isize {
        ((self.q - other.q).abs()
            + (self.q + self.r - other.q - other.r).abs()
            + (self.r - other.r).abs())
            / 2
    }

    fn distance_from_center(self) -> isize {
        cmp::max(cmp::max(self.q, self.r), -self.q - self.r)
    }
}

impl Add for AxialCoordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            q: self.q + other.q,
            r: self.r + other.r,
        }
    }
}

impl TryFrom<&str> for AxialCoordinate {
    type Error = String;

    fn try_from(direction: &str) -> Result<Self, Self::Error> {
        match direction {
            "n" => Ok(AxialCoordinate { q: 0, r: -1 }),
            "ne" => Ok(AxialCoordinate { q: 1, r: -1 }),
            "se" => Ok(AxialCoordinate { q: 1, r: 0 }),
            "s" => Ok(AxialCoordinate { q: 0, r: 1 }),
            "sw" => Ok(AxialCoordinate { q: -1, r: 1 }),
            "nw" => Ok(AxialCoordinate { q: -1, r: 0 }),
            _ => Err(format!("Unknown direction `{}`", direction)),
        }
    }
}

#[aoc(day11, part1)]
pub fn end_distance(input: &str) -> isize {
    input
        .split(',')
        .map(|dir| AxialCoordinate::try_from(dir).unwrap())
        .fold(AxialCoordinate::default(), |acc, dir| acc + dir)
        .distance(AxialCoordinate::default())
}

#[aoc(day11, part2)]
pub fn max_distance(input: &str) -> isize {
    let movement = input
        .split(',')
        .map(|dir| AxialCoordinate::try_from(dir).unwrap());
    let mut loc = AxialCoordinate::default();
    let mut max = 0;
    for dir in movement {
        loc = loc + dir;
        max = cmp::max(loc.distance_from_center(), max);
    }
    max
}
