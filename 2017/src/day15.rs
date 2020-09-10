use std::iter::Iterator;

#[aoc_generator(day15)]
fn seeds(_input: &str) -> (u64, u64) {
    (873, 583)
}

#[aoc(day15, part1)]
fn generators_match_lowest_16_bits(input: &(u64, u64)) -> usize {
    let mut a = input.0;
    let mut b = input.1;
    let mut matched = 0;
    for _ in 0..40_000_000 {
        a = a * 16807 % 2147483647;
        b = b * 48271 % 2147483647;
        if a & 0xFFFF == b & 0xFFFF {
            matched += 1;
        }
    }
    matched
}

struct Generator {
    seed: u64,
    multiplier: u64,
    divisor: u64,
}

impl Generator {
    fn new(seed: u64, multiplier: u64, divisor: u64) -> Self {
        Generator {
            seed,
            multiplier,
            divisor,
        }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.seed = self.seed * self.multiplier % 2147483647;
            if self.seed % self.divisor == 0 {
                break;
            }
        }
        Some(self.seed)
    }
}

#[aoc(day15, part2, alt1)]
fn generators_match_picky_iter(input: &(u64, u64)) -> usize {
    Generator::new(input.0, 16807, 4)
        .zip(Generator::new(input.1, 48271, 8))
        .take(5_000_000)
        .filter(|(a, b)| a & 0xFFFF == b & 0xFFFF)
        .count()
}

#[aoc(day15, part2, alt2)]
fn generators_match_picky(input: &(u64, u64)) -> usize {
    let mut a = input.0;
    let mut b = input.1;
    let mut matched = 0;
    for _ in 0..5_000_000 {
        loop {
            a = a * 16807 % 2147483647;
            if a % 4 == 0 {
                break;
            }
        }
        loop {
            b = b * 48271 % 2147483647;
            if b % 8 == 0 {
                break;
            }
        }
        if a & 0xFFFF == b & 0xFFFF {
            matched += 1;
        }
    }
    matched
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(588, generators_match_lowest_16_bits(&(65, 8921)));
    }

    #[test]
    fn part2() {
        assert_eq!(309, generators_match_picky(&(65, 8921)));
    }
}
