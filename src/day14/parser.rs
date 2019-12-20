use nom::character::complete::{alpha1, digit1, multispace0, space1};
use std::str;

named!(
    number<&str, usize>,
    map!(digit1, |s| usize::from_str_radix(s, 10).unwrap())
);

named!(chemical<&str, String>,
    map!(alpha1, |s| String::from(s))
);

named!(
    requirement<&str, (String, usize)>,
    do_parse!(opt!(tag!(", ")) >> num: number >> space1 >> chem: chemical >> ((chem, num)))
);

named!(
    formula<&str, ((String, usize), Vec<(String, usize)>)>,
    do_parse!(
        multispace0
            >> needed: many1!(requirement)
            >> tag!(" => ")
            >> wanted: requirement
            >> (wanted, needed)
    )
);

named!(pub formulae<&str, Vec<((String, usize), Vec<(String, usize)>)>>,
    many1!(complete!(formula))
);

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse_number() {
        assert_eq!(number("4"), Ok(("", 4)));
    }
}
