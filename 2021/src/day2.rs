#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    let mut x = 0;
    let mut y = 0;
    for mut it in input.lines().map(|l| l.split_ascii_whitespace()) {
        match (
            it.next().unwrap(),
            it.next().unwrap().parse::<usize>().unwrap(),
        ) {
            ("forward", n) => x += n,
            ("down", n) => y += n,
            ("up", n) => y -= n,
            (_, _) => unreachable!(),
        }
    }

    x * y
}

#[aoc(day2, part2)]
fn part2(input: &str) -> isize {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for mut it in input.lines().map(|l| l.split_ascii_whitespace()) {
        match (
            it.next().unwrap(),
            it.next().unwrap().parse::<isize>().unwrap(),
        ) {
            ("forward", n) => {
                x += n;
                y += n * aim;
            }
            ("down", n) => aim += n,
            ("up", n) => aim -= n,
            (_, _) => unreachable!(),
        }
    }

    x * y
}
