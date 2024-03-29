#![feature(iter_intersperse)]
extern crate aoc_runner;
#[macro_use]
extern crate aoc_runner_derive;
extern crate aoc;
extern crate crypto;
extern crate nom;
extern crate num;
extern crate priority_queue;

pub mod cpu;
pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

aoc_lib! { year = 2019 }
