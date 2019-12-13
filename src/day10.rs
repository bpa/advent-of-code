use num::Integer;
use std::collections::HashSet;

#[aoc_generator(day10)]
pub fn coordinates(input: &str) -> Vec<(usize, usize)> {
    input
        .split_whitespace()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter_map(move |(x, point)| match point {
                    '#' => Some((x, y)),
                    _ => None,
                })
        })
        .collect()
}

#[aoc(day10, part1)]
pub fn where_to_build(input: &[(usize, usize)]) -> usize {
    let mut visible: Vec<HashSet<(isize, isize)>> = input.iter().map(|_| HashSet::new()).collect();
    for first in 0..input.len() {
        for second in (first + 1)..input.len() {
            let one = input[first];
            let two = input[second];
            let x = one.0 as isize - two.0 as isize;
            let y = one.1 as isize - two.1 as isize;
            let d = x.gcd(&y);
            visible[first].insert((x / d, y / d));
            visible[second].insert((-x / d, -y / d));
        }
    }

    visible.iter().map(|asteroid| asteroid.len()).max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generator() {
        assert_eq!(
            vec!((0, 0), (1, 0), (2, 0), (1, 1), (1, 2), (2, 2)),
            coordinates("###\n.#.\n.##")
        );
    }
}
