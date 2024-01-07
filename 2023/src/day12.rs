use std::collections::HashMap;

use aoc::*;
use nom::character::complete;

fn arrangements(
    input: &[u8],
    springs: &Vec<u32>,
    ind: usize,
    spr: usize,
    cache: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if let Some(ans) = cache.get(&(ind, spr)) {
        return *ans;
    }
    let ans = _arrangements(input, springs, ind, spr, cache);
    cache.insert((ind, spr), ans);
    ans
}

macro_rules! find {
    ($in:expr, $c:literal) => {
        $in.into_iter().find(|&&c| c == $c)
    };
}

fn _arrangements(
    input: &[u8],
    springs: &Vec<u32>,
    ind: usize,
    spr: usize,
    cache: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if spr == springs.len() {
        return if ind < input.len() && find!(input[ind..], b'#').is_some() {
            0
        } else {
            1
        };
    }

    // Check for enough chars left in string for springs to fit
    if springs[spr..].iter().sum::<u32>() as usize + springs.len() - spr - 1 > input.len() - ind {
        return 0;
    }

    if input[ind] == b'.' {
        return arrangements(input, springs, ind + 1, spr, cache);
    }

    let mut a = 0;
    let s = springs[spr];
    let end = ind + s as usize;
    if (end == input.len() || input[end] != b'#') && find!(&input[ind..end], b'.').is_none() {
        a = arrangements(input, springs, end + 1, spr + 1, cache);
    }

    if input[ind] == b'#' {
        return a;
    }
    a + arrangements(input, springs, ind + 1, spr, cache)
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            arrangements(
                parts.next().unwrap().as_bytes(),
                &comb(complete::u32)(parts.next().unwrap()),
                0,
                0,
                &mut HashMap::new(),
            )
        })
        .sum()
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            let pattern = parts.next().unwrap();
            let springs = comb(complete::u32)(parts.next().unwrap());
            let count = springs.len();
            arrangements(
                [pattern, pattern, pattern, pattern, pattern]
                    .join("?")
                    .as_bytes(),
                &springs.into_iter().cycle().take(count * 5).collect(),
                0,
                0,
                &mut HashMap::new(),
            )
        })
        .inspect(|x| debug!("{}", x))
        .sum()
}
