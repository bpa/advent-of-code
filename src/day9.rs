use cpu::{read_memory, Intcode};
use std::iter::once;

#[aoc_generator(day9)]
fn load(input: &str) -> Vec<isize> {
    read_memory(input)
}

#[aoc(day9, part1)]
pub fn boost(input: &[isize]) -> isize {
    let mut last: isize = -1;
    for code in Intcode::new(Vec::from(input), &mut once(1)) {
        println!("{}", code);
        last = code;
    }
    last
}

#[aoc(day9, part2)]
pub fn get_coordinates(input: &[isize]) -> Option<isize> {
    Intcode::new(Vec::from(input), &mut once(2)).last()
}
