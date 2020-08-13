use cpu::{read_memory, Intcode};
use std::iter::once;

#[aoc_generator(day5)]
fn load(input: &str) -> Vec<isize> {
    read_memory(input)
}

#[aoc(day5, part1)]
pub fn run_system_1(m: &Vec<isize>) -> isize {
    let mut last: isize = -1;
    for code in Intcode::new(m.clone(), &mut once(1)) {
        println!("{}", code);
        last = code;
    }
    last
}

#[aoc(day5, part2)]
pub fn run_system_5(m: &Vec<isize>) -> Option<isize> {
    Intcode::new(m.clone(), &mut once(5)).next()
}
