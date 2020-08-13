use std::convert::TryFrom;
use std::default::Default;
use std::ops::Add;

#[derive(Debug, PartialEq, Default)]
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
pub fn fuel_requirements(input: &str) -> isize {
    input
        .split(',')
        .map(|dir| AxialCoordinate::try_from(dir).unwrap())
        .fold(AxialCoordinate::default(), |acc, dir| acc + dir)
        .distance(AxialCoordinate::default())
}
