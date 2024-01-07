use num::Integer;
use std::{
    collections::{hash_map::Entry, HashMap},
    ops::Index,
};

#[derive(Debug)]
pub struct Node<'a> {
    name: &'a str,
    l: usize,
    r: usize,
}

struct GhostMap<'a> {
    directions: &'a str,
    lookup: HashMap<&'a str, usize>,
    nodes: Vec<Node<'a>>,
}

impl<'a> GhostMap<'a> {
    fn get(&self, k: &'a str) -> &Node {
        let i = self.lookup.get(k).unwrap();
        &self.nodes[*i]
    }

    fn find_or_create(&mut self, k: &'a str) -> usize {
        match self.lookup.entry(k) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(e) => {
                let i = self.nodes.len();
                self.nodes.push(Node {
                    name: k,
                    l: 0,
                    r: 0,
                });
                e.insert(i);
                i
            }
        }
    }
}

impl<'a> From<&'a str> for GhostMap<'a> {
    fn from(value: &'a str) -> Self {
        let mut lines = value.lines();
        let mut index: Self = GhostMap {
            directions: lines.next().unwrap(),
            lookup: HashMap::new(),
            nodes: Vec::new(),
        };
        lines.next();
        for line in lines {
            let i = index.find_or_create(&line[..3]);
            let l = index.find_or_create(&line[7..10]);
            let r = index.find_or_create(&line[12..15]);
            let n = &mut index.nodes[i];
            n.l = l;
            n.r = r;
        }
        index
    }
}

impl<'a> Index<usize> for GhostMap<'a> {
    type Output = Node<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> isize {
    let map = GhostMap::from(input);
    let mut steps = 0;
    let mut n = map.get("AAA");
    for dir in map.directions.chars().cycle() {
        steps += 1;
        n = if dir == 'R' { &map[n.r] } else { &map[n.l] };
        if n.name == "ZZZ" {
            return steps;
        }
    }
    0
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> u64 {
    let map = GhostMap::from(input);
    map.nodes
        .iter()
        .filter_map(|mut n| {
            if !n.name.ends_with('A') {
                return None;
            }
            let mut steps: u64 = 0;
            for dir in map.directions.chars().cycle() {
                steps += 1;
                n = if dir == 'R' { &map[n.r] } else { &map[n.l] };
                if n.name.ends_with('Z') {
                    return Some(steps);
                }
            }
            None
        })
        .reduce(|a, b| a.lcm(&b))
        .unwrap()
}
