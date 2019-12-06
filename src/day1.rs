#[aoc(day1, part1)]
pub fn fuel_requirements(input: &str) -> u64 {
    input
        .split_whitespace()
        .map(|mass| mass.parse::<u64>().unwrap())
        .map(|mass| mass / 3 - 2)
        .sum()
}
