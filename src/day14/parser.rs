use nom::character::complete::{alpha1, digit1, multispace0, space1};
use std::str::FromStr;

named!(
    number<usize>,
    map!(digit1, |s| usize::from_str_radix(s, 10))
);

// named!(
//     requirement<(&[u8], usize)>,
//     do_parse!(num: map!(digit1,  >> space1 >> chem: alpha1 >> ((chem, num)))
// );

// named!(
//     formula<((&str, usize), Vec<(&str, usize)>)>,
//     do_parse!(
//         opt!(tag!(", "))
//             >> needed: many!(requirement)
//             >> tag(" => ")
//             >> wanted: requirement
//             >> ((wanted, needed))
//     )
// );

named!(pub formulae<Vec<((&str, usize), Vec<(&str, usize)>)>>,
    many1!(complete!(formula))
);
