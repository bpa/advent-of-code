use std::collections::HashSet;

use aoc::{Grid, Neighbors, Point2D};

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum GridEntry {
    Number(u32),
    Symbol(char),
    #[default]
    Empty,
}

impl From<u8> for GridEntry {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::Empty,
            c if c.is_ascii_digit() => Self::Number((c as char).to_digit(10).unwrap()),
            c => Self::Symbol(c as char),
        }
    }
}

#[aoc_generator(day3)]
pub fn parse(input: &str) -> Grid<GridEntry> {
    let mut grid = Grid::from(input);
    grid.set_neighbors(Neighbors::Eight);
    grid
}

fn adjacent(point: Point2D<GridEntry>) -> Vec<u32> {
    let mut seen = HashSet::new();
    point
        .adjacent()
        .into_iter()
        .filter_map(|mut c| {
            if !matches!(c.get(), GridEntry::Number(_)) || seen.contains(&c) {
                return None;
            }
            while let Some(l) = c.w() {
                match l.get() {
                    GridEntry::Number(_) => {
                        c = l;
                    }
                    _ => break,
                }
            }
            let mut num = 0;
            let mut d = Some(c);
            while let Some(ref digit) = d {
                match digit.get() {
                    GridEntry::Number(n) => {
                        num *= 10;
                        num += n;
                        seen.insert(digit.clone());
                        d = digit.e()
                    }
                    _ => break,
                }
            }
            Some(num)
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(grid: &Grid<GridEntry>) -> u32 {
    grid.points()
        .filter(|p| matches!(p.get(), GridEntry::Symbol(_)))
        .flat_map(|p| adjacent(p))
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(grid: &Grid<GridEntry>) -> u32 {
    grid.points()
        .filter(|p| {
            if let GridEntry::Symbol(s) = p.get() {
                s == '*'
            } else {
                false
            }
        })
        .map(|p| adjacent(p))
        .filter_map(|parts| {
            if parts.len() == 2 {
                Some(parts[0] * parts[1])
            } else {
                None
            }
        })
        .sum()
}
