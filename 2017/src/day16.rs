use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, digit1};
use nom::combinator::map;
use nom::sequence::{pair, tuple};
use nom::IResult;

#[derive(Debug, PartialEq)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(usize, usize),
}

fn number(input: &str) -> IResult<&str, usize> {
    map(digit1, |d: &str| d.parse::<usize>().unwrap())(input)
}

fn partner(input: &str) -> IResult<&str, usize> {
    map(anychar, |d: char| (d as u32 - 'a' as u32) as usize)(input)
}

fn dance_move(input: &str) -> IResult<&str, Move> {
    alt((
        map(pair(tag("s"), number), |(_, amount)| Move::Spin(amount)),
        map(
            tuple((tag("x"), number, tag("/"), number)),
            |(_, a, _, b)| Move::Exchange(a, b),
        ),
        map(
            tuple((tag("p"), partner, tag("/"), partner)),
            |(_, a, _, b)| Move::Partner(a, b),
        ),
    ))(input)
}

#[aoc(day16, part1)]
fn dance(input: &str) -> String {
    let mut start = 0;
    let mut line = [0; 16];
    let mut index = [0; 16];
    for i in 0..16 {
        line[i] = i;
        index[i] = i;
    }

    for step in input.split(",") {
        match dance_move(step).unwrap().1 {
            Move::Spin(n) => start = (16 + start - n) % 16,
            Move::Exchange(a, b) => {
                let line_a = (a + start) % 16;
                let line_b = (b + start) % 16;
                let (ind_a, ind_b) = (line[line_a], line[line_b]);
                let tmp = index[ind_a];
                index[ind_a] = index[ind_b];
                index[ind_b] = tmp;
                let tmp = line[line_a];
                line[line_a] = line[line_b];
                line[line_b] = tmp;
            }
            Move::Partner(ind_a, ind_b) => {
                let (line_a, line_b) = (index[ind_a], index[ind_b]);
                let tmp = index[ind_a];
                index[ind_a] = index[ind_b];
                index[ind_b] = tmp;
                let tmp = line[line_a];
                line[line_a] = line[line_b];
                line[line_b] = tmp;
            }
        }
    }

    String::from_utf8(
        line[start..16]
            .iter()
            .chain(line[0..start].iter())
            .map(|c| (*c as u32 + 'a' as u32) as u8)
            .collect(),
    )
    .unwrap()
}

#[aoc(day16, part2)]
fn full_dance(input: &str) -> String {
    let moves: Vec<Move> = input
        .split(",")
        .map(|step| dance_move(step).unwrap().1)
        .collect();

    let mut start = 0;
    let mut line = [0; 16];
    let mut index = [0; 16];
    for i in 0..16 {
        line[i] = i;
        index[i] = i;
    }

    for step in moves.iter().cycle().take(1_000_000_000) {
        match step {
            Move::Spin(n) => start = (16 + start - n) % 16,
            Move::Exchange(a, b) => {
                let line_a = (a + start) % 16;
                let line_b = (b + start) % 16;
                let (ind_a, ind_b) = (line[line_a], line[line_b]);
                let tmp = index[ind_a];
                index[ind_a] = index[ind_b];
                index[ind_b] = tmp;
                line[line_a] = ind_b;
                line[line_b] = ind_a;
            }
            Move::Partner(ind_a, ind_b) => {
                let (line_a, line_b) = (index[*ind_a], index[*ind_b]);
                index[*ind_a] = line_b;
                index[*ind_b] = line_a;
                let tmp = line[line_a];
                line[line_a] = line[line_b];
                line[line_b] = tmp;
            }
        }
    }

    String::from_utf8(
        line[start..16]
            .iter()
            .chain(line[0..start].iter())
            .map(|c| (*c as u32 + 'a' as u32) as u8)
            .collect(),
    )
    .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(Move::Spin(5), dance_move("s5").unwrap().1);
        assert_eq!(Move::Exchange(3, 4), dance_move("x3/4").unwrap().1);
        assert_eq!(Move::Partner(4, 1), dance_move("pe/b").unwrap().1);
    }

    #[test]
    fn part1() {
        assert_eq!("abcdefghijklmnop", dance("s0"));
        assert_eq!("pabcdefghijklmno", dance("s1"));
        assert_eq!("pabdcefghijklmno", dance("s1,x3/4"));
        assert_eq!("paedcbfghijklmno", dance("s1,x3/4,pe/b"));
    }
}
