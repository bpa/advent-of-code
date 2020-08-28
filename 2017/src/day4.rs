use std::collections::HashSet;

#[aoc(day4, part1)]
pub fn valid_passphrases(input: &str) -> usize {
    let phrases = input
        .lines()
        .map(|pass| pass.split_ascii_whitespace().collect::<Vec<&str>>().len());
    let unique = input.lines().map(|pass| {
        pass.split_ascii_whitespace()
            .collect::<HashSet<&str>>()
            .len()
    });
    phrases
        .zip(unique)
        .map(|(a, b)| if a == b { 1 } else { 0 })
        .sum()
}

#[aoc(day4, part2)]
pub fn valid_passphrases_anagram(input: &str) -> usize {
    input
        .lines()
        .map(|pass| {
            pass.split_ascii_whitespace()
                .map(|phrase| {
                    let mut chars = String::into_bytes(phrase.into());
                    chars.sort();
                    String::from_utf8(chars).unwrap()
                })
                .collect::<Vec<String>>()
        })
        .map(|words| words.len() == words.iter().collect::<HashSet<&String>>().len())
        .map(|ok| if ok { 1 } else { 0 })
        .sum()
}
