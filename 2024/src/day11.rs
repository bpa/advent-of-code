use aoc::*;
use memoize::memoize;
use nom::character::complete;

#[aoc_generator(day11)]
pub fn parse(input: &str) -> Vec<u64> {
    comb(complete::u64)(input)
}

#[aoc(day11, part1)]
pub fn part1(input: &Vec<u64>) -> u64 {
    input.iter().map(|stone| blink(*stone, 25)).sum()
}

#[memoize]
fn blink(stone: u64, count: usize) -> u64 {
    if stone == 0 {
        return match count {
            1 => 1,
            _ => blink(1, count - 1),
        };
    }
    if let Some((l, r)) = even_digits(stone) {
        return match count {
            1 => 2,
            _ => blink(l, count - 1) + blink(r, count - 1),
        };
    }
    return match count {
        1 => 1,
        _ => blink(stone * 2024, count - 1),
    };
}

fn even_digits(n: u64) -> Option<(u64, u64)> {
    let str = n.to_string();
    if str.len() % 2 == 1 {
        return None;
    }
    let l = str[0..str.len() / 2].parse::<u64>().unwrap();
    let r = str[str.len() / 2..].parse::<u64>().unwrap();
    Some((l, r))
}

#[aoc(day11, part2)]
pub fn part2(input: &Vec<u64>) -> u64 {
    input.iter().map(|stone| blink(*stone, 75)).sum()
}

#[cfg(test)]
mod test {
    use super::blink;
    use crate::day11::*;

    #[test]
    fn blinker() {
        assert_eq!(1, blink(0, 1));
        assert_eq!(1, blink(1, 1));
        assert_eq!(2, blink(10, 1));
        assert_eq!(2, blink(99, 1));
        assert_eq!(1, blink(999, 1));
    }

    #[test]
    fn splitter() {
        assert_eq!(even_digits(17), Some((1, 7)));
        assert_eq!(even_digits(3), None);
        assert_eq!(even_digits(100), None);
        assert_eq!(even_digits(1000), Some((10, 0)));
    }
}
