use cpu::Intcode;
use std::collections::HashSet;
use std::iter::empty;
use std::num::ParseIntError;

#[aoc_generator(day13)]
fn read_memory(input: &str) -> Result<Vec<isize>, ParseIntError> {
    input.split(',').map(|num| num.parse::<isize>()).collect()
}

#[aoc(day13, part1)]
pub fn arcade_blocks(m: &Vec<isize>) -> usize {
    Intcode::new(m.clone(), &mut empty())
        .collect::<Vec<isize>>()
        .chunks(3)
        .filter_map(|block| match block[2] {
            2 => Some((block[0], block[1])),
            _ => None,
        })
        .collect::<HashSet<(isize, isize)>>()
        .len()
}

#[aoc(day13, part2)]
pub fn breakout_in_space(_: &[isize]) -> &'static str {
    "run day13 executable and play/watch for result"
}
