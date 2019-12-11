use std::collections::HashMap;

pub struct Orbit {
    center: String,
    satellite: String,
}

#[aoc_generator(day6)]
pub fn orbitlist<'a>(input: &'a str) -> Vec<Orbit> {
    input
        .split_whitespace()
        .map(|o| {
            let mut planets = o.split(")");
            Orbit {
                center: planets.next().unwrap().to_string(),
                satellite: planets.next().unwrap().to_string(),
            }
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn orbits(input: &[Orbit]) -> usize {
    let mut count: usize = 0;
    let mut system: HashMap<String, usize> = HashMap::new();
    for orbit in input {
        let links: usize = match system.get(&orbit.center) {
            Some(satillites) => *satillites,
            None => 1,
        };
        let satillites: usize = match system.get(&orbit.satellite) {
            Some(s) => *s,
            None => 1,
        };
        system.insert(orbit.satellite.to_string(), satillites + links);
        count = count + links;
    }
    count
}
