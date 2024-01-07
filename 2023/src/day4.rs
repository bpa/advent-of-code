use std::{collections::HashSet, iter::FromIterator};

use nom::{
    character::complete::{self, multispace1},
    multi::many1,
    sequence::preceded,
    IResult,
};

fn ints(input: &str) -> IResult<&str, Vec<u32>> {
    many1(preceded(multispace1, complete::u32))(input)
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.split(':').nth(1).unwrap())
        .map(|l| {
            let sets = l
                .split('|')
                .map(ints)
                .map(|x| x.unwrap().1)
                .map(HashSet::from_iter)
                .collect::<Vec<HashSet<u32>>>();
            let exp = sets[0].intersection(&sets[1]).count() as u32;
            2usize.pow(exp - 1)
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    let mut cards = input
        .lines()
        .map(|l| l.split(':').nth(1).unwrap())
        .map(|l| {
            let sets = l
                .split('|')
                .map(ints)
                .map(|x| x.unwrap().1)
                .map(HashSet::from_iter)
                .collect::<Vec<HashSet<u32>>>();
            sets[0].intersection(&sets[1]).count()
        })
        .map(|n| (n, 1))
        .collect::<Vec<(usize, usize)>>();

    let mut total = 0;
    for i in 0..cards.len() {
        let (won, copies) = cards[i];
        total += copies;
        for j in 1..=won {
            cards[i + j].1 += copies;
        }
    }
    total
}
