use std::collections::HashMap;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> isize {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let mut nums = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<isize>().unwrap());
        left.push(nums.next().unwrap());
        right.push(nums.next().unwrap());
    }
    left.sort_unstable();
    right.sort_unstable();
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut left = HashMap::new();
    let mut right = HashMap::new();
    for line in input.lines() {
        let mut nums = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<usize>().unwrap());
        left.entry(nums.next().unwrap())
            .and_modify(|n| *n += 1)
            .or_insert(1);
        right
            .entry(nums.next().unwrap())
            .and_modify(|n| *n += 1)
            .or_insert(1);
    }
    let mut answer = 0;
    for (k, cnt) in left.iter() {
        if let Some(n) = right.get(k) {
            answer += k * cnt * n;
        }
    }
    answer
}
