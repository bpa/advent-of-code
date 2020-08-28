#[aoc_generator(day5)]
fn memory(input: &str) -> Vec<isize> {
    input.lines().map(|l| l.parse::<isize>().unwrap()).collect()
}

#[aoc(day5, part1)]
fn escape(input: &Vec<isize>) -> usize {
    let mut input = input.clone();
    let mut steps = 0;
    let mut i: isize = 0;
    while i >= 0 && (i as usize) < input.len() {
        steps = steps + 1;
        let instruction = i;
        i = instruction + input[instruction as usize];
        input[instruction as usize] += 1;
    }
    steps
}

#[aoc(day5, part2)]
fn escape2(input: &Vec<isize>) -> usize {
    let mut input = input.clone();
    let mut steps = 0;
    let mut i: isize = 0;
    while i >= 0 && (i as usize) < input.len() {
        steps = steps + 1;
        let instruction = i;
        let jump = input[i as usize];
        i = instruction + jump;
        if jump >= 3 {
            input[instruction as usize] -= 1;
        } else {
            input[instruction as usize] += 1;
        }
    }
    steps
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(escape(&vec![0, 3, 0, 1, -3]), 5);
    }

    #[test]
    fn part2() {
        assert_eq!(escape2(&vec![0, 3, 0, 1, -3]), 10);
    }
}
