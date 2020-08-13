use super::Coordinate;
use nom::character::complete::{digit1, multispace0};

named!(signed_int<&str, isize>,
    map_res!(
        recognize!(
            pair!(
                opt!(alt!(tag!("+") | tag!("-"))),
                digit1
            )
        )
        , str::parse::<isize>
    )
);

named!(point<&str, Coordinate>, 
    do_parse!(
        multispace0   >>
        tag!("<x=")   >>
        x: signed_int >>
        tag!(", y=")  >> 
        y: signed_int >> 
        tag!(", z=")  >>
        z: signed_int >>
        tag!(">")     >>
        multispace0   >>
        (Coordinate { x: x, y: y, z: z })
    )
);

named!(pub coordinates<&str, Vec<Coordinate>>,
    many1!(complete!(point))
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn number() {
        assert_eq!(signed_int("1"), Ok(("", 1)));
        assert_eq!(signed_int("-1"), Ok(("", -1)));
    }
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
