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
    let ids_vec = input.lines().collect::<Vec<&str>>();
    let ids = ids_vec.as_slice();
    for i in 0..ids.len() {
        for j in i + 1..ids.len() {
            if ids[i]
                .chars()
                .zip(ids[j].chars())
                .filter(|(a, b)| a != b)
                .count()
                == 1
            {
                return ids[i]
                    .chars()
                    .zip(ids[j].chars())
                    .filter_map(|(a, b)| (a == b).then_some(a))
                    .collect();
            }
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
