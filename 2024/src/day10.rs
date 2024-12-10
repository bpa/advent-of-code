use std::collections::HashSet;

use aoc::*;

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
    Grid::from_digits(input).points().map(trailheads).sum()
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
    Grid::from_digits(input).points().map(paths_to_peak).sum()
}

fn trailheads(head: Point2D<u32>) -> usize {
    if head.get() != 0 {
        return 0;
    }
    let mut stack = Vec::with_capacity(1024);
    let mut visited = HashSet::new();
    let mut peaks = HashSet::new();
    stack.push(head);
    while let Some(p) = stack.pop() {
        if p.get() == 9 {
            peaks.insert(p.clone());
        }
        visited.insert(p.clone());
        for n in p.adjacent() {
            if n.get() == p.get() + 1 {
                stack.push(n);
            }
        }
    }
    peaks.len()
}

fn paths_to_peak(head: Point2D<u32>) -> usize {
    if head.get() != 0 {
        return 0;
    }
    let mut peaks = 0;
    let mut stack = Vec::with_capacity(1024);
    stack.push(head);
    while let Some(p) = stack.pop() {
        if p.get() == 9 {
            peaks += 1;
            continue;
        }
        for n in p.adjacent() {
            if n.get() == p.get() + 1 {
                stack.push(n);
            }
        }
    }
    peaks
}
