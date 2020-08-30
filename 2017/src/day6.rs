use std::collections::{BTreeMap, BTreeSet};

#[aoc_generator(day6)]
fn memory_banks(input: &str) -> Vec<u8> {
    input
        .split_ascii_whitespace()
        .map(|num| num.parse::<u8>().unwrap())
        .collect()
}

#[aoc(day6, part1)]
fn cycles(input: &Vec<u8>) -> usize {
    assert!(input.len() <= 16);
    let mut seen = BTreeSet::new();
    let mut banks = input.clone();
    while seen.insert(to_hash(&banks)) {
        redistribute(&mut banks);
    }
    seen.len()
}

#[aoc(day6, part2)]
fn cycle_length(input: &Vec<u8>) -> usize {
    assert!(input.len() <= 16);
    let mut seen = BTreeMap::new();
    let mut banks = input.clone();
    loop {
        match seen.insert(to_hash(&banks), seen.len()) {
            Some(last_seen) => return seen.len() - last_seen,
            None => redistribute(&mut banks),
        }
    }
}

fn redistribute(banks: &mut Vec<u8>) {
    let mut max = 0u8;
    let mut max_ind = 0;
    for (i, v) in banks.iter().enumerate() {
        if *v > max {
            max = *v;
            max_ind = i;
        }
    }

    banks[max_ind] = 0;
    while max > 0 {
        max_ind = (max_ind + 1) % banks.len();
        banks[max_ind] += 1;
        max -= 1;
    }
}

fn to_hash(banks: &Vec<u8>) -> u128 {
    let mut hash: u128 = 0;
    for v in banks {
        hash <<= 8;
        hash += *v as u128;
    }
    hash
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_part1() {
        assert_eq!(cycles(&vec![0, 2, 7, 0]), 5);
    }

    #[test]
    fn sample_part2() {
        assert_eq!(cycle_length(&vec![0, 2, 7, 0]), 4);
    }
}
