use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

type Rulebook = HashMap<usize, HashSet<usize>>;

fn is_valid(pages: &Vec<usize>, rules: &Rulebook) -> bool {
    for i in 0..pages.len() - 1 {
        match rules.get(&pages[i]) {
            Some(after) => {
                for n in &pages[i + 1..] {
                    if !after.contains(n) {
                        return false;
                    }
                }
            }
            None => return false,
        }
    }
    true
}

#[aoc_generator(day5)]
pub fn parse(input: &str) -> (Rulebook, Vec<Vec<usize>>) {
    let mut parts = input.split("\n\n");
    let mut rules = HashMap::new();
    parts.next().unwrap().lines().for_each(|l| {
        let nums: Vec<usize> = l.split("|").map(|n| n.parse::<usize>().unwrap()).collect();
        let after = rules.entry(nums[0]).or_insert_with(HashSet::new);
        after.insert(nums[1]);
    });
    let pages = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            l.split(",")
                .filter_map(|n| n.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .collect();
    (rules, pages)
}

#[aoc(day5, part1)]
pub fn part1((rules, pages): &(Rulebook, Vec<Vec<usize>>)) -> usize {
    pages
        .into_iter()
        .filter(|nums| is_valid(nums, rules))
        .map(|nums| nums[nums.len() / 2])
        .sum()
}

#[aoc(day5, part2)]
pub fn part2((rules, pages): &(Rulebook, Vec<Vec<usize>>)) -> usize {
    pages
        .into_iter()
        .filter(|nums| !is_valid(nums, rules))
        .map(|nums| order(nums, rules))
        .map(|nums| nums[nums.len() / 2])
        .sum()
}

fn order(pages: &Vec<usize>, rules: &Rulebook) -> Vec<usize> {
    let mut fixed = pages.clone();
    fixed.sort_by(|a, b| match rules.get(a) {
        Some(e) => match e.contains(b) {
            true => Ordering::Less,
            false => Ordering::Greater,
        },
        None => Ordering::Greater,
    });
    fixed
}
