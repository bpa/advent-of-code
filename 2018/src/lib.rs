#![feature(bool_to_option, map_first_last)]
extern crate aoc_runner;
#[macro_use]
extern crate aoc_runner_derive;
#[macro_use]
extern crate lazy_static;
extern crate nom;
extern crate paste;

use nom::character::complete::{digit1, r#char};
use nom::combinator::{map_res, opt, recognize};
use nom::sequence::pair;
use nom::IResult;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

aoc_lib! { year = 2018 }

pub fn number(input: &str) -> IResult<&str, isize> {
    map_res(recognize(pair(opt(char('-')), digit1)), |d: &str| {
        d.parse::<isize>()
    })(input)
}
