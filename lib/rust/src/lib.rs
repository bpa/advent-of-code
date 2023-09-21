use std::{fmt, str::FromStr};

use nom::{
    character::complete::{digit1, one_of},
    combinator::{opt, recognize},
    sequence::tuple,
    IResult,
};

pub fn signed_int<T: FromStr>(input: &str) -> IResult<&str, T>
where
    T::Err: fmt::Debug,
{
    let (input, number) = recognize(tuple((opt(one_of("+-")), digit1)))(input)?;
    Ok((input, number.parse::<T>().unwrap()))
}

pub fn unsigned_int<T: FromStr>(input: &str) -> IResult<&str, T>
where
    T::Err: fmt::Debug,
{
    let (input, number) = recognize(digit1)(input)?;
    Ok((input, number.parse::<T>().unwrap()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn number() {
        assert_eq!(signed_int("+1"), Ok(("", 1)));
        assert_eq!(signed_int("-1"), Ok(("", -1)));
    }

    #[test]
    fn parse_number() {
        assert_eq!(unsigned_int("4"), Ok(("", 4)));
    }
}
