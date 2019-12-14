use std::collections::HashMap;

#[aoc_generator(day6)]
pub fn orbitlist<'a>(input: &'a str) -> Vec<(&str, &str)> {
    input
        .split_whitespace()
        .map(|o| {
            let mut planets = o.split(")");
            (planets.next().unwrap(), planets.next().unwrap())
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn orbits(input: &[(&str, &str)]) -> usize {
    let mut system: HashMap<&str, Node> = HashMap::new();

    for orbit in input {
        let satellite = system.entry(orbit.1).or_insert_with(|| Node::new(orbit.1));
        let planet = system.entry(orbit.0).or_insert_with(|| Node::new(orbit.0));
        planet.satellites.push(satellite);
    }
    system.values_mut().map(|s| s.num_routes()).sum()
}

enum Visited {
    No,
    InProgress,
    Yes,
}

struct Node<'a> {
    center: &'a str,
    satellites: Vec<&'a mut Node<'a>>,
    visited: Visited,
    routes: usize,
}

impl<'a> Node<'a> {
    fn new(center: &'a str) -> Self {
        Node {
            center: center,
            satellites: Vec::new(),
            visited: Visited::No,
            routes: 0,
        }
    }
    fn num_routes(&mut self) -> usize {
        match self.visited {
            Visited::Yes => self.routes,
            Visited::InProgress => panic!("Cycle in graph detected"),
            Visited::No => {
                self.visited = Visited::InProgress;
                self.routes = self.satellites.len()
                    + self
                        .satellites
                        .iter_mut()
                        .map(|s| s.num_routes())
                        .sum::<usize>();
                self.visited = Visited::Yes;
                self.routes
            }
        }
    }
}
