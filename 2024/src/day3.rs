use std::ops::Add;

use nom::Err;
use nom::IResult;
use nom::InputLength;
use nom::Slice;
use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_until};
use nom::character::complete;
use nom::combinator::opt;
use nom::multi::fold_many0;

enum Inst {
    Do(u32),
    Dont(u32),
    Value(u32),
}

impl Add<Inst> for Inst {
    type Output = Inst;

    fn add(self, rhs: Self) -> Self::Output {
        match rhs {
            Inst::Do(_) => Inst::Do(self.get()),
            Inst::Dont(_) => Inst::Dont(self.get()),
            Inst::Value(x) => match self {
                Inst::Do(y) => Inst::Do(x + y),
                Inst::Dont(_) => self,
                Inst::Value(y) => Inst::Do(x + y),
            },
        }
    }
}

impl Inst {
    fn get(self) -> u32 {
        match self {
            Inst::Do(x) => x,
            Inst::Dont(x) => x,
            Inst::Value(x) => x,
        }
    }
}

fn mul(input: &str) -> IResult<&str, u32> {
    let (input, _) = take_until("mul(")(input)?;
    let (input, _) = take(4usize)(input)?;
    let (input, val) = opt(do_mul)(input)?;
    Ok((input, val.unwrap_or(0)))
}

fn non_consuming_mul(input: &str) -> IResult<&str, Inst> {
    let (input, _) = tag("mul(")(input)?;
    let (input, val) = do_mul(input)?;
    Ok((input, Inst::Value(val)))
}

fn do_mul(input: &str) -> IResult<&str, u32> {
    let (input, x) = complete::u32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = complete::u32(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, x * y))
}

fn inst(input: &str) -> IResult<&str, Inst> {
    let mut i = input;
    loop {
        match alt((inst_do, inst_dont, non_consuming_mul))(i) {
            Ok((i1, o)) => {
                return Ok((i1, o));
            }
            Err(Err::Error(e)) => {
                if i.input_len() < 4 {
                    return Err(Err::Error(e));
                }
                i = i.slice(1..);
            }
            Err(e) => return Err(e),
        }
    }
}

fn inst_do(input: &str) -> IResult<&str, Inst> {
    let (input, _) = tag("do()")(input)?;
    Ok((input, Inst::Do(0)))
}

fn inst_dont(input: &str) -> IResult<&str, Inst> {
    let (input, _) = tag("don't()")(input)?;
    Ok((input, Inst::Dont(0)))
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    fold_many0(mul, || 0, |acc, val| acc + val)(input)
        .unwrap()
        .1
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    fold_many0(inst, || Inst::Do(0), |acc, val| acc + val)(input)
        .unwrap()
        .1
        .get()
}
