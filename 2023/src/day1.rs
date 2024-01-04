use regex::Regex;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .map(|c: Vec<u32>| c.first().unwrap() * 10 + c.last().unwrap())
        .sum()
}

fn find(re: &Regex, line: &str) -> usize {
    match re.captures(line).unwrap().get(1).unwrap().as_str() {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        n => n.parse::<usize>().unwrap(),
    }
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let first = Regex::new(r".*?(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let last = Regex::new(r".*(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    input
        .lines()
        .map(|line| find(&first, line) * 10 + find(&last, line))
        .sum()
}
