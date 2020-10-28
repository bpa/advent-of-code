#[aoc_generator(day9)]
fn setup(input: &str) -> (usize, usize) {
    let mut tokens = input.split_ascii_whitespace();
    (
        tokens.next().unwrap().parse::<usize>().unwrap(),
        tokens.nth(5).unwrap().parse::<usize>().unwrap(),
    )
}

#[derive(Clone, Default)]
struct Marble {
    left: usize,
    right: usize,
}

#[aoc(day9, part1)]
fn high_score((players, last): &(usize, usize)) -> usize {
    let mut scores = vec![0; *players];
    let mut bag = vec![Marble::default(); *last + 1];
    let mut marbles = bag.as_mut_slice();

    let mut current = 0;
    for (player, marble) in (0..*players).cycle().zip(1..=*last) {
        if marble % 23 == 0 {
            scores[player] += marble;
            for _ in 0..7 {
                current = marbles[current].left;
            }
            scores[player] += current;
            marbles[marbles[current].left].right = marbles[current].right;
            marbles[marbles[current].right].left = marbles[current].left;
            current = marbles[current].right;
        } else {
            current = marbles[current].right;
            marbles[marble].right = marbles[current].right;
            marbles[current].right = marble;
            marbles[marble].left = current;
            marbles[marbles[marble].right].left = marble;
            current = marble;
        }
    }
    *scores.iter().max().unwrap()
}

#[aoc(day9, part2)]
fn super_high_score((players, last): &(usize, usize)) -> usize {
    high_score(&(*players, last * 100))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn input() {
        assert_eq!(
            (10, 1618),
            setup("10 players; last marble is worth 1618 points")
        )
    }

    #[test]
    fn part1() {
        assert_eq!(high_score(&(9, 25)), 32);
        assert_eq!(high_score(&(10, 1618)), 8317);
        assert_eq!(high_score(&(13, 7999)), 146373);
        assert_eq!(high_score(&(17, 1104)), 2764);
        assert_eq!(high_score(&(21, 6111)), 54718);
        assert_eq!(high_score(&(30, 5807)), 37305);
    }
}
