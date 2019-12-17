use cpu::Intcode;
use std::iter::once;
use std::num::ParseIntError;

#[aoc_generator(day9)]
fn read_memory(input: &str) -> Result<Vec<isize>, ParseIntError> {
    input.split(',').map(|num| num.parse::<isize>()).collect()
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
