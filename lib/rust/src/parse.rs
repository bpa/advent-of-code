use std::ops::RangeFrom;

use nom::{Err, InputLength, Parser, Slice};

pub fn comb<I, O, F>(mut f: F) -> impl FnMut(I) -> Vec<O>
where
    I: Clone + InputLength + Slice<RangeFrom<usize>>,
    F: Parser<I, O, (I, nom::error::ErrorKind)>,
{
    move |mut i: I| {
        let mut res = Vec::new();
        loop {
            match f.parse(i.clone()) {
                Ok((i1, o)) => {
                    res.push(o);
                    i = i1
                }
                Err(Err::Error(_)) => {
                    if i.input_len() < 2 {
                        return res;
                    }
                    i = i.slice(1..);
                }
                Err(_) => return res,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::character::complete;

    #[test]
    fn parse_numbers() {
        assert_eq!(
            comb(complete::i32)("1 2,3|4hi5x-6 "),
            vec![1, 2, 3, 4, 5, -6]
        );
        assert_eq!(comb(complete::i32)("1-2,3"), vec![1, -2, 3])
    }
}
