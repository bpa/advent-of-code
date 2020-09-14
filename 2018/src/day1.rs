use std::collections::HashSet;

#[aoc_generator(day1)]
fn numbers(input: &str) -> Vec<isize> {
    input.lines().map(|n| n.parse::<isize>().unwrap()).collect()
}

#[aoc(day1, part1)]
fn sum_them(input: &Vec<isize>) -> isize {
    input.iter().sum()
}

#[aoc(day1, part2)]
fn first_repeated_frequency(input: &Vec<isize>) -> isize {
    let mut seen = HashSet::new();
    let mut freq = 0;
    for i in input.iter().cycle() {
        if !seen.insert(freq) {
            return freq;
        }
        freq += i;
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generator() {
        assert_eq!(vec![1, -2, 3, -4], numbers("+1,-2, +3 ,-4"));
    }

    #[test]
    fn part2() {
        assert_eq!(0, first_repeated_frequency(&vec![1, -1]));
        assert_eq!(10, first_repeated_frequency(&vec![3, 3, 4, -2, -4]));
        assert_eq!(5, first_repeated_frequency(&vec![-6, 3, 8, 5, -6]));
        assert_eq!(14, first_repeated_frequency(&vec![7, 7, -2, -7, -4]));
    }
}
