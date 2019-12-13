use cpu::run;
use std::num::ParseIntError;

#[aoc_generator(day5)]
fn read_memory(input: &str) -> Result<Vec<isize>, ParseIntError> {
    input.split(',').map(|num| num.parse::<isize>()).collect()
}

#[aoc(day5, part1)]
pub fn run_system_1(m: &Vec<isize>) -> isize {
    let code = m.clone();

    run(code, vec![1], &mut |i, val| println!("{}: {}", i, val))
}

#[aoc(day5, part2)]
pub fn run_system_5(m: &Vec<isize>) -> isize {
    let code = m.clone();

    run(code, vec![5], &mut |i, val| println!("{}: {}", i, val))
}
