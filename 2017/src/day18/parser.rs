use super::instructions::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, digit1, multispace0, multispace1};
use nom::combinator::{map, opt};
use nom::multi::many1;
use nom::IResult;
use paste::paste;
use std::boxed::Box;

enum Field {
    Register(isize),
    Value(isize),
}

pub fn parse(input: &str) -> IResult<&str, Vec<Box<dyn Instruction>>> {
    many1(instruction)(input)
}

macro_rules! parse_inst {
    ($func:ident, $type:ident) => {
        parse_inst! { $func, stringify!($func), $type }
    };

    ($func:ident, $type:ident, $arg:ident) => {
        parse_inst! { $func, stringify!($func), $type, $arg }
    };

    ($func:ident, $tag:expr, $type:ident) => {
        parse_inst! { $func, $tag, $type, input, r, {
            Ok((input, Box::new($type { r: r as usize })))
        }}
    };

    ($func:ident, $tag:expr, $type:ident, $arg:ident) => {
        parse_inst! { $func, $tag, $type, input, r, {
            paste! {
            let (input, _) = multispace1(input)?;
            let (input, $arg) = field(input)?;
            match $arg {
                Field::Register(a) => Ok((input, Box::new([<$type R>] { r: r as usize, a }))),
                Field::Value(a) => Ok((input, Box::new($type { r: r as usize, a }))),
            }}
        }}
    };

    ($func:ident, $tag:expr, $type:ident, $input:ident, $r:ident, $body:block) => {
        fn $func($input: &str) -> IResult<&str, Box<dyn Instruction>> {
            let ($input, _) = tag($tag)($input)?;
            let ($input, _) = multispace1($input)?;
            let ($input, $r) = register($input)?;
            $body
        }
    };
}

parse_inst! {add, Add, a}
parse_inst! {jgz, Jgz, a}
parse_inst! {mul, Mul, a}
parse_inst! {modulus, "mod", Mod, a}
parse_inst! {rcv, Rcv }
parse_inst! {snd, Snd }
parse_inst! {set, Set, a}

fn instruction(input: &str) -> IResult<&str, Box<dyn Instruction>> {
    let (input, _) = multispace0(input)?;
    alt((add, jgz, mul, modulus, rcv, snd, set))(input)
}

fn field(input: &str) -> IResult<&str, Field> {
    alt((
        map(number, |r| Field::Value(r)),
        map(register, |r| Field::Register(r)),
    ))(input)
}

fn number(input: &str) -> IResult<&str, isize> {
    let (input, neg): (&str, Option<&str>) = opt(tag("-"))(input)?;
    let (input, value) = map(digit1, |num: &str| num.parse::<isize>().unwrap())(input)?;
    match neg {
        Some(_) => Ok((input, -value)),
        None => Ok((input, value)),
    }
}

fn register(input: &str) -> IResult<&str, isize> {
    let (input, r) = anychar(input)?;
    Ok((input, r as isize - 'a' as isize))
}
