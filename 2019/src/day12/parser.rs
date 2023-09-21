use super::Coordinate;
use aoc::signed_int;
use nom::{bytes::complete::tag, character::complete::multispace0, multi::many1, IResult};

fn coord(input: &str) -> IResult<&str, Coordinate> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("<x=")(input)?;
    let (input, x) = signed_int(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y) = signed_int(input)?;
    let (input, _) = tag(", z=")(input)?;
    let (input, z) = signed_int(input)?;
    let (input, _) = tag(">")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, Coordinate { x: x, y: y, z: z }))
}

pub fn coordinates(input: &str) -> IResult<&str, Vec<Coordinate>> {
    many1(coord)(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(
            coordinates("<x=0, y=1, z=2>\n<x=-2, y=0, z=-0>\n"),
            Ok((
                "",
                vec![
                    Coordinate { x: 0, y: 1, z: 2 },
                    Coordinate { x: -2, y: 0, z: 0 }
                ]
            ))
        );
    }
}
