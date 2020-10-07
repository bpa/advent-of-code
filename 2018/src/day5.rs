use std::cmp::min;

lazy_static! {
    static ref LOOKUP: [u8; 256] = {
        let mut lookup = [0; 256];
        for (lower, upper) in ('A'..='Z').into_iter().zip(('a'..='z').into_iter()) {
            lookup[upper as usize] = lower as u8;
            lookup[lower as usize] = upper as u8;
        }
        lookup
    };
}

#[aoc(day5, part1)]
fn units_remaining(input: &str) -> usize {
    let mut chain = Vec::with_capacity(input.len());
    for b in input.bytes() {
        if chain.last() == Some(&LOOKUP[b as usize]) {
            chain.pop();
        } else {
            chain.push(b);
        }
    }

    chain.len()
}

#[aoc(day5, part2)]
fn shortest_reduced(input: &str) -> usize {
    let mut initial_reduction = Vec::with_capacity(input.len());
    for b in input.bytes() {
        if initial_reduction.last() == Some(&LOOKUP[b as usize]) {
            initial_reduction.pop();
        } else {
            initial_reduction.push(b);
        }
    }

    let mut shortest = usize::MAX;
    let mut chain = Vec::with_capacity(initial_reduction.len());
    for exclude in 'a'..='z' {
        let lc = exclude as u8;
        let uc = LOOKUP[exclude as usize];

        chain.clear();
        for b in initial_reduction.iter() {
            if *b == lc || *b == uc {
                continue;
            }

            if chain.last() == Some(&LOOKUP[*b as usize]) {
                chain.pop();
            } else {
                chain.push(*b);
            }
        }
        shortest = min(shortest, chain.len());
    }

    shortest
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(0, units_remaining("aA"));
        assert_eq!(0, units_remaining("Aa"));
        assert_eq!(0, units_remaining("zZ"));
        assert_eq!(0, units_remaining("Zz"));
        assert_eq!(0, units_remaining("abBA"));
        assert_eq!(1, units_remaining("a"));
        assert_eq!(1, units_remaining("aAa"));
        assert_eq!(6, units_remaining("aabAAB"));
        assert_eq!(10, units_remaining("dabAcCaCBAcCcaDA"))
    }
}
