use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, multispace0};
use nom::combinator::{map, opt, recognize, value};
use nom::multi::many0;
use nom::IResult;
use std::collections::BTreeSet;

fn number(input: &str) -> IResult<&str, isize> {
    let (input, _) = multispace0(input)?;
    let (input, _) = opt(tag(","))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, sign) = alt((value(1, tag("+")), value(-1, tag("-"))))(input)?;
    map(recognize(digit1), move |n: &str| {
        sign * n.parse::<isize>().unwrap()
    })(input)
}

#[aoc_generator(day1)]
fn numbers(input: &str) -> Vec<isize> {
    many0(number)(input).unwrap().1
}

#[aoc(day1, part1)]
fn sum_them(input: &Vec<isize>) -> isize {
    input.iter().sum()
}

#[aoc(day1, part2)]
fn first_repeated_frequency(input: &Vec<isize>) -> isize {
    let mut seen = BTreeSet::new();
    let mut freq = 0;
    for i in input.iter().cycle() {
        if !seen.insert(freq) {
            return freq;
        }
        freq += i;
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generator() {
        assert_eq!(vec![1, -2, 3, -4], numbers("+1,-2, +3 ,-4"));
    }

    #[test]
    fn part2() {
        assert_eq!(0, first_repeated_frequency(&vec![1, -1]));
        assert_eq!(10, first_repeated_frequency(&vec![3, 3, 4, -2, -4]));
        assert_eq!(5, first_repeated_frequency(&vec![-6, 3, 8, 5, -6]));
        assert_eq!(14, first_repeated_frequency(&vec![7, 7, -2, -7, -4]));
    }
}
