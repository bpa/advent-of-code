use regex::Regex;

#[aoc(day2, part1)]
fn check_passwords(input: &str) -> usize {
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    input
        .lines()
        .filter(|line| {
            let group = re.captures(line).unwrap();
            let min = group.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let max = group.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let c = group.get(3).unwrap().as_str().chars().next().unwrap();
            let password = group.get(4).unwrap().as_str();
            let frequency = password.chars().filter(|letter| *letter == c).count();
            frequency >= min && frequency <= max
        })
        .count()
}

#[aoc(day2, part2)]
fn check_passwords_xor(input: &str) -> usize {
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    input
        .lines()
        .filter(|line| {
            let group = re.captures(line).unwrap();
            let one = group.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let two = group.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let c = group.get(3).unwrap().as_str().chars().next().unwrap();
            let mut password = group.get(4).unwrap().as_str().chars();
            let first = password.nth(one - 1).unwrap_or_default() == c;
            let second = password.nth(two - one - 1).unwrap_or_default() == c;
            first ^ second
        })
        .count()
}
