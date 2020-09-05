#[aoc(day9, part1)]
fn group_score(input: &str) -> usize {
    let mut score = 0;
    let mut depth = 0;
    let mut chars = input.chars();
    'parse: loop {
        'group: loop {
            match chars.next() {
                Some(c) => match c {
                    '{' => {
                        depth += 1;
                        score += depth;
                    }
                    '}' => depth -= 1,
                    '<' => break 'group,
                    _ => {}
                },
                None => break 'parse,
            }
        }

        'garbage: loop {
            match chars.next() {
                Some(c) => match c {
                    '!' => {
                        if let None = chars.next() {
                            break 'parse;
                        }
                    }
                    '>' => break 'garbage,
                    _ => {}
                },
                None => break 'parse,
            }
        }
    }
    score
}

#[aoc(day9, part2)]
fn garbage(input: &str) -> usize {
    let mut garbage_count = 0;
    let mut chars = input.chars();
    'parse: loop {
        'group: loop {
            match chars.next() {
                Some(c) => match c {
                    '<' => break 'group,
                    _ => {}
                },
                None => break 'parse,
            }
        }

        'garbage: loop {
            match chars.next() {
                Some(c) => match c {
                    '!' => {
                        if let None = chars.next() {
                            break 'parse;
                        }
                    }
                    '>' => break 'garbage,
                    _ => garbage_count += 1,
                },
                None => break 'parse,
            }
        }
    }
    garbage_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(group_score("{}"), 1);
        assert_eq!(group_score("{{{}}}"), 6);
        assert_eq!(group_score("{{},{}}"), 5);
        assert_eq!(group_score("{{{},{},{{}}}}"), 16);
        assert_eq!(group_score("{<a>,<a>,<a>,<a>}"), 1);
        assert_eq!(group_score("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
        assert_eq!(group_score("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
        assert_eq!(group_score("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
    }

    #[test]
    fn part2() {
        assert_eq!(garbage("<>"), 0);
        assert_eq!(garbage("<random characters>"), 17);
        assert_eq!(garbage("<<<<>"), 3);
        assert_eq!(garbage("<{!>}>"), 2);
        assert_eq!(garbage("<!!>"), 0);
        assert_eq!(garbage("<!!!>>"), 0);
        assert_eq!(garbage("<{o\"i!a,<{i<a>"), 10);
    }
}
