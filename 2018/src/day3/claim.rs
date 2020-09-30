use nom::bytes::complete::tag;
use nom::character::complete::{digit1, multispace0};
use nom::combinator::map;
use nom::IResult;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Claim {
    pub id: usize,
    pub bounds: Rect,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize,
}

impl Rect {
    pub fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        Rect { x1, y1, x2, y2 }
    }
}

impl Claim {
    pub fn overlaps(&self, other: &Claim) -> bool {
        self.bounds.overlaps(&other.bounds)
    }
}

impl Rect {
    pub fn overlaps(&self, other: &Rect) -> bool {
        self.x1 < other.x2 && self.x2 > other.x1 && self.y1 < other.y2 && self.y2 > other.y1
    }
}

fn parse_claim(input: &str) -> IResult<&str, Claim> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("#")(input)?;
    let (input, id) = map(digit1, |num: &str| num.parse::<usize>().unwrap())(input)?;
    let (input, _) = tag(" @ ")(input)?;
    let (input, x) = map(digit1, |num: &str| num.parse::<usize>().unwrap())(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = map(digit1, |num: &str| num.parse::<usize>().unwrap())(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, w) = map(digit1, |num: &str| num.parse::<usize>().unwrap())(input)?;
    let (input, _) = tag("x")(input)?;
    let (input, h) = map(digit1, |num: &str| num.parse::<usize>().unwrap())(input)?;
    Ok((
        input,
        Claim {
            id,
            bounds: Rect {
                x1: x,
                y1: y,
                x2: x + w,
                y2: y + h,
            },
        },
    ))
}

impl From<&str> for Claim {
    fn from(input: &str) -> Self {
        parse_claim(input).unwrap().1
    }
}
