use aoc::*;
use nom::character::complete;

fn ways_to_win((t, d): (i64, i64)) -> i64 {
    let mut x = ((t * t - 4 * d) as f64).sqrt();
    if x.fract() == 0.0 {
        x -= 0.5;
    }
    ((t as f64 + x) / 2f64) as i64 - ((t as f64 - x) / 2f64) as i64
}
#[aoc(day6, part1)]
pub fn part1(input: &str) -> i64 {
    let mut lines = input.lines();
    let times = comb(complete::i64)(lines.next().unwrap());
    let distances = comb(complete::i64)(lines.next().unwrap());
    times
        .into_iter()
        .zip(distances)
        .map(ways_to_win)
        .inspect(|x| debug!("{}", x))
        .product()
}

fn number(input: Option<&str>) -> i64 {
    input.unwrap().chars().fold(0, |acc, c| {
        if let Some(v) = char::to_digit(c, 10) {
            acc * 10 + v as i64
        } else {
            acc
        }
    })
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> i64 {
    let mut lines = input.lines();
    ways_to_win((number(lines.next()), number(lines.next())))
}
