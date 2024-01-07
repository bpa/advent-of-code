use std::cmp::max;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{self, alpha0, multispace0},
    combinator::opt,
    multi::{fold_many_m_n, many0},
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Default)]
pub struct StoneGrab {
    red: u32,
    green: u32,
    blue: u32,
}

fn grab(input: &str) -> IResult<&str, StoneGrab> {
    let (input, stones) = fold_many_m_n(
        1,
        100,
        preceded(
            tuple((opt(tag(",")), multispace0)),
            tuple((complete::u32, multispace0, alpha0)),
        ),
        StoneGrab::default,
        |mut stones: StoneGrab, stone| {
            match stone.2 {
                "red" => stones.red = stone.0,
                "green" => stones.green = stone.0,
                "blue" => stones.blue = stone.0,
                color => panic!("Unknown stone color: {}", color),
            }
            stones
        },
    )(input)?;
    let (input, _) = opt(tag(";"))(input)?;
    Ok((input, stones))
}

fn game(input: &str) -> IResult<&str, Vec<StoneGrab>> {
    let (input, _) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;
    many0(grab)(input)
}

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<Vec<StoneGrab>> {
    input.lines().map(|x| game(x).unwrap().1).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Vec<StoneGrab>]) -> usize {
    input
        .iter()
        .enumerate()
        .filter_map(|game| {
            if game
                .1
                .iter()
                .all(|grab| grab.red <= 12 && grab.green <= 13 && grab.blue <= 14)
            {
                Some(game.0 + 1)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Vec<StoneGrab>]) -> u32 {
    input
        .iter()
        .map(|game| {
            game.iter().fold(StoneGrab::default(), |mut req, grab| {
                req.red = max(req.red, grab.red);
                req.green = max(req.green, grab.green);
                req.blue = max(req.blue, grab.blue);
                req
            })
        })
        .map(|game| game.red * game.green * game.blue)
        .sum()
}
