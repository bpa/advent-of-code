use core::str;
use std::collections::HashMap;

struct Towels {
    available: HashMap<u8, Vec<Vec<u8>>>,
}

impl From<&str> for Towels {
    fn from(value: &str) -> Self {
        let mut available = HashMap::new();
        for pattern in value.split(", ") {
            available
                .entry(pattern.as_bytes()[0])
                .or_insert_with(|| Vec::new())
                .push(pattern.bytes().collect())
        }
        Self { available }
    }
}

impl Towels {
    fn can_make(&self, pattern: &str) -> bool {
        let data: Vec<u8> = pattern.bytes().collect();
        let mut stack = Vec::new();
        let end = data.len();
        let mut seen = vec![false; pattern.len() + 1];
        stack.push(0);
        while let Some(pos) = stack.pop() {
            if pos == end {
                return true;
            }
            if let Some(candidates) = self.available.get(&data[pos]) {
                'next: for candidate in candidates {
                    for (i, c) in candidate.iter().enumerate() {
                        match data.get(pos + i) {
                            Some(v) => {
                                if v != c {
                                    continue 'next;
                                }
                            }
                            None => continue 'next,
                        }
                    }
                    let loc = pos + candidate.len();
                    if !seen[loc] {
                        stack.push(pos + candidate.len());
                        seen[loc] = true;
                    }
                }
            }
        }
        false
    }

    fn arrangements(&self, pattern: &str) -> usize {
        let data: Vec<u8> = pattern.bytes().collect();
        let mut counts = vec![0; data.len()];
        for pos in (0..data.len()).rev() {
            let mut count = 0;
            if let Some(candidates) = self.available.get(&data[pos]) {
                'next: for candidate in candidates {
                    for (i, c) in candidate.iter().enumerate() {
                        match data.get(pos + i) {
                            Some(v) => {
                                if v != c {
                                    continue 'next;
                                }
                            }
                            None => continue 'next,
                        }
                    }
                    count += match counts.get(pos + candidate.len()) {
                        Some(n) => *n,
                        None => 1,
                    }
                }
            }
            counts[pos] = count;
        }
        counts[0]
    }
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let towels = Towels::from(lines.next().unwrap());
    lines.next();
    lines.filter(|&pattern| towels.can_make(pattern)).count()
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let towels = Towels::from(lines.next().unwrap());
    lines.next();
    lines.map(|pattern| towels.arrangements(pattern)).sum()
}
