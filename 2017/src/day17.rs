#[aoc_generator(day17)]
fn number(input: &str) -> usize {
    input.parse::<usize>().unwrap()
}

#[aoc[day17, part1]]
fn value_after_2017(jump: &usize) -> usize {
    let mut buf = Vec::with_capacity(2018);
    let mut pos = 0;
    buf.push(0);
    for next in 1..=2017 {
        pos = (pos + jump) % next + 1;
        buf.insert(pos, next);
    }
    buf[(pos + 1) % buf.len()]
}

#[aoc[day17, part2]]
fn value_after_5_m(jump: &usize) -> usize {
    let mut answer = 0;
    let mut pos = 0;
    for next in 1..=50_000_000 {
        pos = (pos + jump) % next + 1;
        if pos == 1 {
            answer = next;
        }
    }
    answer
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(638, value_after_2017(&3));
    }
}
