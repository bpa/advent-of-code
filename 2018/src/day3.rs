use nom::bytes::complete::tag;
use nom::character::complete::{digit1, multispace0};
use nom::combinator::map;
use nom::IResult;

#[derive(Debug, PartialEq)]
struct Claim {
    id: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
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
    Ok((input, Claim { id, x, y, w, h }))
}

#[aoc_generator(day3)]
fn parse_claims(input: &str) -> Vec<Claim> {
    input
        .lines()
        .map(|claim| parse_claim(claim).unwrap().1)
        .collect()
}

#[aoc(day3, part1)]
fn multi_claims(claims: &Vec<Claim>) -> usize {
    let mut whole_fabric = vec![0usize; 1000 * 1000];
    let fabric = whole_fabric.as_mut_slice();
    let mut overlapping = 0;
    for claim in claims {
        for x in claim.x..claim.x + claim.w {
            for y in claim.y..claim.y + claim.h {
                let i = x + y * 1000;
                fabric[i] += 1;
                if fabric[i] == 2 {
                    overlapping += 1;
                }
            }
        }
    }
    overlapping
}

#[aoc(day3, part2)]
fn the_one_good_claim(claims: &Vec<Claim>) -> usize {
    let mut whole_fabric = vec![0usize; 1000 * 1000];
    let fabric = whole_fabric.as_mut_slice();
    let mut overlapping = 0;
    for claim in claims {
        for x in claim.x..claim.x + claim.w {
            for y in claim.y..claim.y + claim.h {
                let i = x + y * 1000;
                fabric[i] += 1;
                if fabric[i] == 2 {
                    overlapping += 1;
                }
            }
        }
    }
    overlapping
}

#[cfg(test)]
mod test {
    use super::*;

    const CLAIMS: &str = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";

    #[test]
    fn parse() {
        assert_eq!(
            vec![
                Claim {
                    id: 1,
                    x: 1,
                    y: 3,
                    w: 4,
                    h: 4
                },
                Claim {
                    id: 2,
                    x: 3,
                    y: 1,
                    w: 4,
                    h: 4
                },
                Claim {
                    id: 3,
                    x: 5,
                    y: 5,
                    w: 2,
                    h: 2
                }
            ],
            parse_claims(CLAIMS)
        )
    }

    #[test]
    fn part1() {
        assert_eq!(4, multi_claims(&parse_claims(CLAIMS)))
    }

    #[test]
    fn part1() {
        assert_eq!(3, the_one_good_claim(&parse_claims(CLAIMS)))
    }
}
