#[aoc_generator(day2)]
pub fn to_digits(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn checksum(input: &Vec<Vec<isize>>) -> isize {
    input
        .iter()
        .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
        .sum()
}

#[aoc(day2, part2)]
pub fn divisible_checksum(input: &Vec<Vec<isize>>) -> isize {
    input
        .iter()
        .map(|row| {
            for i in 0..row.len() {
                let left = row[i];
                for j in i + 1..row.len() {
                    let right = row[j];
                    if left < right {
                        if right % left == 0 {
                            return right / left;
                        }
                    } else {
                        if left % right == 0 {
                            return left / right;
                        }
                    }
                }
            }
            0
        })
        .sum()
}
