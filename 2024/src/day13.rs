use aoc::*;
use nom::character::complete;

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    x: i64,
    y: i64,
}

#[derive(Debug)]
pub struct Machine {
    a: Vector,
    b: Vector,
    prize: Vector,
}

#[aoc_generator(day13)]
pub fn parse(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|m| comb(complete::i64)(m))
        .map(|toks| Machine {
            a: Vector {
                x: toks[0],
                y: toks[1],
            },
            b: Vector {
                x: toks[2],
                y: toks[3],
            },
            prize: Vector {
                x: toks[4],
                y: toks[5],
            },
        })
        .collect()
}

#[aoc(day13, part1)]
pub fn part1(input: &Vec<Machine>) -> i64 {
    input.iter().filter_map(brute_force).sum()
}

fn brute_force(m: &Machine) -> Option<i64> {
    for a in 1..=100 {
        let target_x = m.prize.x - m.a.x * a;
        if target_x % m.b.x == 0 {
            let b = target_x / m.b.x;
            let tokens = a * 3 + b;
            if m.a.y * a + m.b.y * b == m.prize.y {
                return Some(tokens);
            }
        }
    }
    None
}

#[aoc(day13, part2)]
pub fn part2(input: &Vec<Machine>) -> i64 {
    input.iter().filter_map(solve).sum()
}

fn solve(m: &Machine) -> Option<i64> {
    let prize = Vector {
        x: m.prize.x + 10000000000000,
        y: m.prize.y + 10000000000000,
    };
    let numerator = prize.y * m.a.x - prize.x * m.a.y;
    let denominator = m.b.y * m.a.x - m.b.x * m.a.y;
    if denominator == 0 || numerator % denominator != 0 {
        return None;
    }
    let b = numerator / denominator;
    if b < 0 {
        return None;
    }
    let diff_x = prize.x - b * m.b.x;
    if diff_x % m.a.x > 0 {
        return None;
    }
    let a = diff_x / m.a.x;
    Some(a * 3 + b)
}
