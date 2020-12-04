extern crate aoc_runner;
#[macro_use]
extern crate aoc_runner_derive;
extern crate nom;
extern crate regex;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

use nom::character::complete::{digit1, r#char};
use nom::combinator::{map_res, opt, recognize};
use nom::sequence::pair;
use nom::IResult;

pub fn number(input: &str) -> IResult<&str, isize> {
    map_res(recognize(pair(opt(char('-')), digit1)), |d: &str| {
        d.parse::<isize>()
    })(input)
}

aoc_lib! { year = 2020 }
