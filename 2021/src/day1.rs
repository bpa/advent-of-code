#[aoc(day1, part1)]
fn part1(input: &str) -> usize {
    let mut count = 0;
    let mut x = 100000;
    for l in input
        .split_ascii_whitespace()
        .map(|l| l.parse::<usize>().unwrap())
    {
        if l > x {
            count += 1;
        }
        x = l;
    }
    count
}

#[aoc(day1, part2)]
fn part2(input: &str) -> usize {
    let mut count = 0;
    let mut x = 100000;
    for l in input
        .split_ascii_whitespace()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .windows(3)
        .map(|r| r.iter().sum())
    {
        if l > x {
            count += 1;
        }
        x = l;
    }
    count
}
