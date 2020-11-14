use self::ops::*;
use self::register::Register;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, alphanumeric1, digit1, multispace0, r#char};
use nom::combinator::{map_res, opt, recognize};
// use nom::error::{ErrorKind, ParseError};
use nom::sequence::pair;
use nom::IResult;
use std::boxed::Box;
use std::cmp::max;
mod ops;
mod register;

pub fn number(input: &str) -> IResult<&str, isize> {
    map_res(recognize(pair(opt(char('-')), digit1)), |d: &str| {
        d.parse::<isize>()
    })(input)
}

fn conditional<'a>(
    input: &'a str,
    var_table: &mut Register,
) -> IResult<&'a str, Box<dyn Condition>> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("if ")(input)?;
    let (input, register) = alphanumeric1(input)?;
    let (input, _) = multispace0(input)?;
    let (input, op) = take_until(" ")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, val) = number(input)?;
    match op {
        ">" => Ok((input, Box::new(GT::new(var_table.get(register), val)))),
        ">=" => Ok((input, Box::new(GTE::new(var_table.get(register), val)))),
        "<" => Ok((input, Box::new(LT::new(var_table.get(register), val)))),
        "<=" => Ok((input, Box::new(LTE::new(var_table.get(register), val)))),
        "==" => Ok((input, Box::new(EQ::new(var_table.get(register), val)))),
        "!=" => Ok((input, Box::new(NE::new(var_table.get(register), val)))),
        // _ => nom::Err(ParseError::from_error_kind(input, ErrorKind::OneOf)),
        _ => panic!("Invalid op: {}", op),
    }
}

fn operation<'a>(input: &'a str, var_table: &mut Register) -> IResult<&'a str, Box<dyn Update>> {
    let (input, _) = multispace0(input)?;
    let (input, reg) = alphanumeric1(input)?;
    let (input, _) = multispace0(input)?;
    let (input, op) = alpha1(input)?;
    let (input, _) = multispace0(input)?;
    let (input, val) = number(input)?;
    let (input, cond) = conditional(input, var_table)?;
    let register = var_table.get(reg);
    match op {
        "inc" => Ok((input, Box::new(Inc::new(register, val, cond)))),
        "dec" => Ok((input, Box::new(Dec::new(register, val, cond)))),
        _ => panic!("Invalid op: {}", op),
    }
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> (usize, Vec<Box<dyn Update>>) {
    let mut var_table = Register::new();
    let instructions = input
        .lines()
        .map(|l| operation(l, &mut var_table).unwrap().1)
        .collect();
    (var_table.len(), instructions)
}

#[aoc(day8, part1)]
fn max_register((register_count, operations): &(usize, Vec<Box<dyn Update>>)) -> isize {
    let mut registers = vec![0; *register_count];
    for op in operations {
        op.update(&mut registers);
    }
    *registers.iter().max().unwrap()
}

#[aoc(day8, part2)]
fn max_register_ever((register_count, operations): &(usize, Vec<Box<dyn Update>>)) -> isize {
    let mut max_value = 0;
    let mut registers = vec![0; *register_count];
    for op in operations {
        max_value = max(max_value, op.update(&mut registers));
    }
    max_value
}

#[cfg(test)]
mod test {
    use super::*;

    const INST: &str =
        "b inc 5 if a > 1\na inc 1 if b < 5\nc dec -10 if a >= 1\nc inc -20 if c == 10";
    const A: usize = 0;
    const B: usize = 1;
    const C: usize = 2;

    #[test]
    fn test_input() {
        let expected: Vec<Box<dyn Update>> = vec![
            Box::new(Inc::new(B, 5, Box::new(GT::new(A, 1)))),
            Box::new(Inc::new(A, 1, Box::new(LT::new(B, 5)))),
            Box::new(Dec::new(C, -10, Box::new(GTE::new(A, 1)))),
            Box::new(Inc::new(C, -20, Box::new(EQ::new(C, 10)))),
        ];
        assert_eq!(
            format!("{:?}", parse_input(INST)),
            format!("{:?}", (3, expected))
        );
    }

    #[test]
    fn part1() {
        assert_eq!(1, max_register(&parse_input(INST)))
    }

    #[test]
    fn part2() {
        assert_eq!(10, max_register_ever(&parse_input(INST)))
    }
}
