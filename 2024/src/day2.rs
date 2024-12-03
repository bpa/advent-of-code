use aoc::*;
use nom::character::complete;

fn is_slowly_moving<'a, T>(mut levels: T) -> Option<bool>
where
    T: Iterator<Item = &'a u32>,
{
    let mut a = levels.next().unwrap();
    let b = levels.next().unwrap();
    let mut clean = true;

    if a <= b || a - b > 3 {
        clean = false;
        let c = levels.next().unwrap();
        if (a > c && a - c <= 3) || (b > c && b - c <= 3) {
            a = c;
        } else {
            return None;
        }
    } else {
        a = b;
    }

    for b in levels {
        if a <= b || a - b > 3 {
            if clean {
                clean = false;
            } else {
                return None;
            }
        } else {
            a = b;
        }
    }

    Some(clean)
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(comb(complete::u32))
        .filter(|levels| {
            is_slowly_moving(levels.iter()).unwrap_or(false)
                || is_slowly_moving(levels.iter().rev()).unwrap_or(false)
        })
        .count()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(comb(complete::u32))
        .filter(|levels| {
            is_slowly_moving(levels.iter())
                .or(is_slowly_moving(levels.iter().rev()))
                .is_some()
        })
        .count()
}
