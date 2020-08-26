use nom::bytes::complete::is_a;
use nom::character::complete::{digit1, multispace0};
use nom::combinator::map;
use nom::multi::{many0, many1, separated_list};
use nom::IResult;
use std::collections::{BTreeSet, VecDeque};

fn dependencies(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, _) = multispace0(input)?;
    let (input, _) = digit1(input)?;
    let (input, _) = many1(is_a(" <->"))(input)?;
    separated_list(
        is_a(", "),
        map(digit1, |n: &str| n.parse::<usize>().unwrap()),
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    many0(dependencies)(input)
}

#[aoc_generator(day12)]
fn pipes(input: &str) -> Vec<Vec<usize>> {
    parse_input(input).unwrap().1
}

#[aoc(day12, part1)]
fn connected(input: &Vec<Vec<usize>>) -> usize {
    let mut queue = VecDeque::new();
    let mut seen = BTreeSet::new();
    queue.push_back(0);
    while let Some(program) = queue.pop_back() {
        if seen.insert(program) {
            for connection in input[program].iter() {
                queue.push_back(*connection);
            }
        }
    }
    seen.len()
}

#[aoc(day12, part2)]
fn groups(input: &Vec<Vec<usize>>) -> usize {
    let mut total_groups = 0;
    let mut all = BTreeSet::new();
    let mut seen = BTreeSet::new();
    let mut queue = VecDeque::new();

    for i in 0..input.len() {
        all.insert(i);
    }

    while let Some(seed) = all.first() {
        total_groups = total_groups + 1;
        seen.clear();
        queue.push_back(*seed);
        while let Some(program) = queue.pop_back() {
            if seen.insert(program) {
                all.remove(&program);
                for connection in input[program].iter() {
                    queue.push_back(*connection);
                }
            }
        }
    }
    total_groups
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "
    0 <-> 2
    1 <-> 1
    2 <-> 0, 3, 4
    3 <-> 2, 4
    4 <-> 2, 3, 6
    5 <-> 6
    6 <-> 4, 5";

    #[test]
    fn test_input() {
        let (_, out) = parse_input(INPUT).unwrap();
        assert_eq!(7, out.len());
        assert_eq!(1, out[0].len());
        assert_eq!(vec![0, 3, 4], out[2]);
    }
}
