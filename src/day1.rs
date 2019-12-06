#[aoc(day1, part1)]
pub fn fuel_requirements(input: &str) -> u64 {
    input
        .split_whitespace()
        .map(|mass| mass.parse::<u64>().unwrap())
        .map(|mass| mass / 3 - 2)
        .sum()
}

#[aoc(day1, part2)]
pub fn fuel_requirements_including_fuel(input: &str) -> i64 {
    input
        .split_whitespace()
        .map(|mass| mass.parse::<i64>().unwrap())
        .map(|mass| {
            let mut total: i64 = 0;
            let mut fuel = mass;
            loop {
                fuel = fuel / 3 - 2;
                if fuel > 0 {
                    total += fuel
                } else {
                    break;
                }
            }
            total
        })
        .sum()
}
