use std::collections::HashMap;
use std::iter::repeat_with;

#[aoc(day2, part1)]
fn sum_them(input: &str) -> usize {
    let mut counts = HashMap::new();
    let mut two = 0;
    let mut three = 0;
    for id in input.lines() {
        counts.clear();
        for c in id.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
        if counts.iter().find(|(_, &v)| v == 2).is_some() {
            two += 1;
        }
        if counts.iter().find(|(_, &v)| v == 3).is_some() {
            three += 1;
        }
    }
    return two * three;
}

#[aoc(day2, part2)]
fn find_the_cloth(input: &str) -> String {
    let ids: Vec<&str> = input.lines().collect();
    let mut letters: Vec<Vec<Vec<usize>>> =
        repeat_with(|| repeat_with(|| Vec::new()).take(26).collect())
            .take(ids[0].len())
            .collect();

    let mut matches = HashMap::new();
    for (entry, id) in ids.iter().enumerate() {
        matches.clear();
        for (c, pos) in id.chars().zip(letters.iter_mut()) {
            let matched = &mut pos[c as usize - 'a' as usize];
            for prev in matched.iter() {
                *matches.entry(*prev).or_insert(0) += 1;
            }
            matched.push(entry);
        }

        if let Some((row, _)) = matches.iter().find(|(_, &v)| v == ids[0].len() - 1) {
            return ids[*row]
                .chars()
                .zip(id.chars())
                .filter_map(|(a, b)| (a == b).then_some(a))
                .collect();
        }
    }
    String::from("")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part2() {
        assert_eq!(
            "fgij",
            find_the_cloth("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz")
        );
    }
}
