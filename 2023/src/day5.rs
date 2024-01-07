use std::{cmp::min, collections::VecDeque};

use aoc::*;
use nom::character::complete;

#[aoc_generator(day5)]
pub fn parse(input: &str) -> (Vec<i64>, Vec<Vec<SeedMap>>) {
    let mut groups = input.split("\n\n");
    let seeds = groups.next().map(comb(complete::i64)).unwrap();
    let transitions = groups
        .map(comb(complete::i64))
        .map(|t| {
            let mut sm: Vec<SeedMap> = t
                .chunks(3)
                .map(|x| SeedMap {
                    start: x[1],
                    end: x[1] + x[2] - 1,
                    diff: x[0] - x[1],
                })
                .collect();
            sm.sort();
            sm
        })
        .collect();
    (seeds, transitions)
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SeedMap {
    start: i64,
    end: i64,
    diff: i64,
}

#[aoc(day5, part1)]
pub fn part1((seeds, transitions): &(Vec<i64>, Vec<Vec<SeedMap>>)) -> i64 {
    seeds
        .iter()
        .map(|s| {
            transitions.iter().fold(*s, |seed, t| {
                for sm in t {
                    if seed >= sm.start && seed <= sm.end {
                        return seed + sm.diff;
                    }
                }
                seed
            })
        })
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
pub fn part2((seeds, transitions): &(Vec<i64>, Vec<Vec<SeedMap>>)) -> i64 {
    let mut smallest = i64::MAX;
    let mut seeds: VecDeque<(usize, i64, i64)> =
        seeds.chunks(2).map(|r| (0, r[0], r[0] + r[1])).collect();
    while let Some((g, mut start, mut end)) = seeds.pop_back() {
        for (i, seed_maps) in transitions.iter().enumerate().skip(g) {
            for t in seed_maps {
                if start >= t.start && start <= t.end {
                    if end <= t.end {
                        start += t.diff;
                        end += t.diff;
                        break;
                    }
                    seeds.push_back((i + 1, start + t.diff, t.end + t.diff));
                    start = t.end + 1;
                }
            }
        }
        smallest = min(smallest, start);
    }
    smallest
}
