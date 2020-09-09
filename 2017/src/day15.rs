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

#[aoc(day15, part2)]
fn generators_match_picky(input: &(u64, u64)) -> usize {
    let a = Generator {
        seed: input.0,
        multiplier: 16807,
        divisor: 4,
    };
    let b = Generator {
        seed: input.1,
        multiplier: 48271,
        divisor: 8,
    };

    a.zip(b)
        .take(5_000_000)
        .filter(|(a, b)| a & 0xFFFF == b & 0xFFFF)
        .count()
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
