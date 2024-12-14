use std::{cmp::Ordering, collections::HashSet};

use aoc::*;
use nom::character::complete;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;
const EQUATOR: i32 = HEIGHT / 2;
const DATE_LINE: i32 = WIDTH / 2;

#[aoc(day14, part1)]
pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(comb(complete::i32))
        .map(|r| location(r, 100))
        .filter_map(quadrant)
        .fold([0; 4], |mut q, r| {
            q[r] += 1;
            q
        })
        .iter()
        .fold(1, |a, &b| a * b)
}

fn location(r: Vec<i32>, seconds: i32) -> (i32, i32) {
    let mut x = (r[0] + r[2] * seconds) % WIDTH;
    let mut y = (r[1] + r[3] * seconds) % HEIGHT;
    if x < 0 {
        x = WIDTH + x;
    }
    if y < 0 {
        y = HEIGHT + y;
    }
    (x, y)
}

fn quadrant((x, y): (i32, i32)) -> Option<usize> {
    match (x.cmp(&DATE_LINE), y.cmp(&EQUATOR)) {
        (Ordering::Less, Ordering::Less) => Some(0),
        (Ordering::Less, Ordering::Greater) => Some(1),
        (Ordering::Greater, Ordering::Less) => Some(2),
        (Ordering::Greater, Ordering::Greater) => Some(3),
        (_, _) => None,
    }
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> i32 {
    let robots: Vec<Vec<i32>> = input.lines().map(comb(complete::i32)).collect();
    let mut floor = HashSet::new();
    'next_second: for i in 0..10000 {
        floor.clear();
        for r in robots.iter() {
            let pos = location(r.to_vec(), i);
            if !floor.insert(pos) {
                continue 'next_second;
            }
        }
        return i;
    }
    0
}
