use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use aoc::*;

//#[aoc_generator(day8)]
//pub fn parse(input: &str) -> &str {
//    input
//}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    let mut towers = HashMap::new();
    let mut antinodes = HashSet::new();
    let grid: Grid<u8> = Grid::from(input);
    for s in grid.points() {
        if s.get() != b'.' {
            towers.entry(s.get()).or_insert_with(Vec::new).push(s);
        }
    }
    for t in towers.values() {
        for towers in t.into_iter().combinations(2) {
            let dx = towers[0].x as isize - towers[1].x as isize;
            let dy = towers[0].y as isize - towers[1].y as isize;
            if let Some(anti) = towers[0].neighbor(dx, dy) {
                antinodes.insert(anti);
            }
            if let Some(anti) = towers[1].neighbor(-dx, -dy) {
                antinodes.insert(anti);
            }
        }
    }
    antinodes.len()
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    let mut towers = HashMap::new();
    let mut antinodes = HashSet::new();
    let grid: Grid<u8> = Grid::from(input);
    for s in grid.points() {
        if s.get() != b'.' {
            towers.entry(s.get()).or_insert_with(Vec::new).push(s);
        }
    }
    for t in towers.values() {
        for towers in t.into_iter().combinations(2) {
            let dx = towers[0].x as isize - towers[1].x as isize;
            let dy = towers[0].y as isize - towers[1].y as isize;
            let mut t = towers[0].clone();
            while let Some(anti) = t.neighbor(-dx, -dy) {
                antinodes.insert(anti.clone());
                t = anti;
            }
            t = towers[1].clone();
            while let Some(anti) = t.neighbor(dx, dy) {
                antinodes.insert(anti.clone());
                t = anti;
            }
        }
    }
    antinodes.len()
}
