use cpu::Intcode;
use std::num::ParseIntError;

#[aoc_generator(day5)]
fn read_memory(input: &str) -> Result<Vec<isize>, ParseIntError> {
    input.split(',').map(|num| num.parse::<isize>()).collect()
}

#[aoc(day5, part1)]
pub fn run_system_1(m: &Vec<isize>) -> isize {
    let mut last: isize = -1;
    for code in Intcode::new(m.clone(), &mut vec![1].into_iter()) {
        println!("{}", code);
        last = code;
    }
    last
}

#[aoc(day5, part2)]
pub fn run_system_5(m: &Vec<isize>) -> Option<isize> {
    Intcode::new(m.clone(), &mut vec![5].into_iter()).next()
}
