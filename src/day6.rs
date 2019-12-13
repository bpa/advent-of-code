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

enum Visited {
    No,
    InProgress,
    Yes,
}

struct Node<'a> {
    center: &'a str,
    satellites: Vec<&'a Node<'a>>,
    visited: Visited,
    routes: usize,
}

#[aoc(day6, part1)]
pub fn orbits(input: &[(&str, &str)]) -> usize {
    let mut system: HashMap<&str, Node> = HashMap::new();

    for orbit in input {
        let satellite = system.entry(orbit.1).or_insert_with(|| Node {
            center: orbit.1,
            satellites: Vec::new(),
            visited: Visited::No,
            routes: 0,
        });

        system
            .entry(orbit.0)
            .and_modify(|e| e.satellites.push(satellite))
            .or_insert_with(|| Node {
                center: orbit.0,
                satellites: vec![satellite],
                visited: Visited::No,
                routes: 0,
            });
    }
    system.values_mut().map(|s| s.find_routes()).sum()
}

impl Node<'_> {
    fn find_routes(&mut self) -> usize {
        match self.visited {
            Visited::Yes => self.routes,
            Visited::InProgress => panic!("Cycle in graph detected"),
            Visited::No => {
                self.visited = Visited::InProgress;
                self.routes = self.satellites.iter_mut().map(|s| s.find_routes()).sum();
                self.visited = Visited::Yes;
                self.routes
            }
        }
    }
}
