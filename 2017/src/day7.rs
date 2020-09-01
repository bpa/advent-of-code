use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, char, digit1, multispace0};
use nom::combinator::{map, opt};
use nom::multi::{many0, separated_list};
use nom::IResult;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq)]
struct Tower {
    name: String,
    weight: isize,
    holding: Vec<String>,
}

impl Tower {
    fn new(name: &str, weight: isize, holding: Vec<String>) -> Self {
        Tower {
            name: String::from(name),
            weight,
            holding,
        }
    }
}

fn word(input: &str) -> IResult<&str, String> {
    map(alphanumeric1, |w| String::from(w))(input)
}
fn tower(input: &str) -> IResult<&str, Tower> {
    let (input, _) = multispace0(input)?;
    let (input, name) = alphanumeric1(input)?;
    let (input, _) = tag(" (")(input)?;
    let (input, weight) = map(digit1, |n: &str| n.parse::<isize>().unwrap())(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = opt(tag(" -> "))(input)?;
    let (input, holding) = separated_list(tag(", "), word)(input)?;
    Ok((input, Tower::new(name, weight, holding)))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Tower>> {
    many0(tower)(input)
}

#[aoc_generator(day7)]
fn parse_tower(input: &str) -> Vec<Tower> {
    parse_input(input).unwrap().1
}

#[aoc(day7, part1)]
fn the_bottom(input: &Vec<Tower>) -> String {
    let mut candidates = HashSet::new();
    let mut standing = HashSet::new();
    for t in input {
        if !standing.contains(&t.name) {
            candidates.insert(&t.name);
        }
        for h in &t.holding {
            candidates.remove(h);
            standing.insert(h);
        }
    }
    (*candidates.iter().next().unwrap()).clone()
}

#[aoc(day7, part2)]
fn proper_weight(input: &Vec<Tower>) -> isize {
    let towers: HashMap<&String, usize> = input
        .iter()
        .enumerate()
        .map(|(i, t)| (&t.name, i))
        .collect();
    let mut visited = vec![false; input.len()];
    let mut totals: Vec<Option<isize>> = vec![None; input.len()];

    let mut work = VecDeque::new();
    for tower in input {
        work.push_back(&tower.name);
        while let Some(name) = work.pop_back() {
            let ind = *towers.get(&name).unwrap();
            if let None = totals[ind] {
                if visited[ind] {
                    if input[ind].holding.len() > 0 {
                        let weights: Vec<(isize, isize)> = input[ind]
                            .holding
                            .iter()
                            .map(|held| towers[held])
                            .map(|n| (totals[n].unwrap(), input[n].weight))
                            .collect();
                        let (one, two): (Vec<(isize, isize)>, Vec<(isize, isize)>) = weights
                            .iter()
                            .partition(|(total, _)| *total == weights[0].0);
                        if two.len() > 0 {
                            return if one.len() == 1 {
                                one[0].1 - one[0].0 + two[0].0
                            } else {
                                two[0].1 - two[0].0 + one[0].0
                            };
                        }
                        totals[ind] =
                            Some(input[ind].weight + weights[0].0 * weights.len() as isize);
                    } else {
                        totals[ind] = Some(input[ind].weight);
                    }
                } else {
                    visited[ind] = true;
                    work.push_back(name);
                    work.extend(input[ind].holding.iter());
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn input() {
        assert_eq!(
            parse_input(include_str!("day7.example.txt")).unwrap().1,
            vec![
                Tower::new("pbga", 66, vec![]),
                Tower::new("xhth", 57, vec![]),
                Tower::new("ebii", 61, vec![]),
                Tower::new("havc", 66, vec![]),
                Tower::new("ktlj", 57, vec![]),
                Tower::new(
                    "fwft",
                    72,
                    vec!["ktlj".into(), "cntj".into(), "xhth".into()]
                ),
                Tower::new("qoyq", 66, vec![]),
                Tower::new(
                    "padx",
                    45,
                    vec!["pbga".into(), "havc".into(), "qoyq".into()]
                ),
                Tower::new(
                    "tknk",
                    41,
                    vec!["ugml".into(), "padx".into(), "fwft".into()]
                ),
                Tower::new("jptl", 61, vec![]),
                Tower::new(
                    "ugml",
                    68,
                    vec!["gyxo".into(), "ebii".into(), "jptl".into()]
                ),
                Tower::new("gyxo", 61, vec![]),
                Tower::new("cntj", 57, vec![]),
            ]
        )
    }

    #[test]
    fn part1() {
        let input = parse_input(include_str!("day7.example.txt")).unwrap().1;
        assert_eq!("tknk", the_bottom(&input));
    }

    #[test]
    fn part2() {
        let input = parse_input(include_str!("day7.example.txt")).unwrap().1;
        assert_eq!(60, proper_weight(&input));
    }
}
