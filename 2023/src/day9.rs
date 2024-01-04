use aoc::*;
use nom::character::complete;

fn predict<F>(history: Vec<i32>, f: F) -> i32
where
    F: Fn(Vec<i32>, i32) -> i32 + Copy,
{
    let mut next = Vec::with_capacity(history.len() - 1);
    for w in history.windows(2) {
        next.push(w[1] - w[0]);
    }
    if next.iter().all(|v| *v == 0) {
        return history[0];
    }
    let n = predict(next, f);
    f(history, n)
}

#[aoc(day9, part1)]
pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(comb(complete::i32))
        .map(|h| predict(h, |h, d| h.last().unwrap() + d))
        .sum()
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(comb(complete::i32))
        .map(|h| predict(h, |h, d| h[0] - d))
        .sum()
}
