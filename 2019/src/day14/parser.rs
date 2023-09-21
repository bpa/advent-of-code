use aoc::unsigned_int;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, space1},
    combinator::{map, opt},
    multi::many1,
    IResult,
};
use std::str;

fn chemical(input: &str) -> IResult<&str, String> {
    map(alpha1, |s| String::from(s))(input)
}

fn requirement(input: &str) -> IResult<&str, (String, usize)> {
    let (input, _) = opt(tag(", "))(input)?;
    let (input, num) = unsigned_int::<usize>(input)?;
    let (input, _) = space1(input)?;
    let (input, chem) = chemical(input)?;
    Ok((input, (chem, num)))
}

fn formula(input: &str) -> IResult<&str, ((String, usize), Vec<(String, usize)>)> {
    let (input, _) = multispace0(input)?;
    let (input, needed) = many1(requirement)(input)?;
    let (input, _) = tag(" => ")(input)?;
    let (input, wanted) = requirement(input)?;
    Ok((input, (wanted, needed)))
}

pub fn formulae(input: &str) -> IResult<&str, Vec<((String, usize), Vec<(String, usize)>)>> {
    many1(formula)(input)
}
