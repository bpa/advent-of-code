use std::collections::HashSet;

#[aoc(day1, part1)]
pub fn sum_to_2020(input: &str) -> usize {
    let mut seen = HashSet::new();
    for entry in input.lines() {
        let item = entry.parse::<usize>().unwrap();
        let pair = 2020 - item;
        if seen.contains(&pair) {
            return item * pair;
        } else {
            seen.insert(item);
        }
    }
    0
}

#[aoc(day1, part2)]
pub fn three_sum_to_2020(input: &str) -> usize {
    let entries: Vec<usize> = input.lines().map(|l| l.parse::<usize>().unwrap()).collect();
    let map: HashSet<usize> = entries.iter().map(|l| *l).collect();
    let slice = entries.as_slice();
    for i in 0..slice.len() {
        for j in (i + 1)..slice.len() {
            let target = 2020 - slice[i] - slice[j];
            if map.contains(&target) {
                return slice[i] * slice[j] * target;
            }
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(514579, sum_to_2020("1721\n979\n366\n299\n675\n1456"))
    }
}
