use crate::number;
use nom::bytes::complete::{tag, take};
use nom::multi::separated_list;
use nom::IResult;
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
struct Coord {
    x: isize,
    y: isize,
    z: isize,
}

impl Coord {
    fn manhatten_distance(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.manhatten_distance() - other.manhatten_distance() {
            0 => Ordering::Equal,
            d @ _ if d < 0 => Ordering::Less,
            _ => Ordering::Greater,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Particle {
    a: Coord,
    v: Coord,
    p: Coord,
    exploded: bool,
}

fn parse_coord(input: &str) -> IResult<&str, Coord> {
    let (input, _) = take(3usize)(input)?;
    let (input, points) = separated_list(tag(","), number)(input)?;
    let (input, _) = take(1usize)(input)?;
    Ok((
        input,
        Coord {
            x: points[0],
            y: points[1],
            z: points[2],
        },
    ))
}

fn parse_particle(input: &str) -> IResult<&str, Particle> {
    let (input, coords) = separated_list(tag(", "), parse_coord)(input)?;
    Ok((
        input,
        Particle {
            a: coords[2],
            v: coords[1],
            p: coords[0],
            exploded: false,
        },
    ))
}

#[aoc_generator(day20)]
fn parse_input(input: &str) -> Vec<Particle> {
    input
        .lines()
        .map(|particle| parse_particle(particle).unwrap().1)
        .collect()
}

#[aoc(day20, part1)]
fn closest_to_origin(particles: &Vec<Particle>) -> usize {
    particles
        .iter()
        .enumerate()
        .min_by_key(|(_, &particle)| particle)
        .map(|(id, _)| id)
        .unwrap()
}

#[aoc(day20, part2)]
fn left_after_collisions_brute(input: &Vec<Particle>) -> usize {
    let mut all_explosions = Vec::new();
    let mut particles: Vec<Particle> = input.clone();
    let mut positions: HashMap<Coord, usize> = HashMap::new();
    for round in 0..50 {
        positions.clear();
        let mut explosions = Vec::new();

        for (i, particle) in particles.iter_mut().enumerate() {
            if particle.exploded {
                continue;
            }
            particle.v.x += particle.a.x;
            particle.v.y += particle.a.y;
            particle.v.z += particle.a.z;
            particle.p.x += particle.v.x;
            particle.p.y += particle.v.y;
            particle.p.z += particle.v.z;
            match positions.entry(particle.p) {
                Entry::Occupied(orig) => {
                    explosions.push(*orig.get());
                    explosions.push(i);
                }
                Entry::Vacant(e) => {
                    e.insert(i);
                }
            }
        }
        for e in explosions {
            all_explosions.push((round, e));
            particles[e].exploded = true;
        }
    }

    particles.iter().filter(|p| p.exploded == false).count()
}

#[cfg(test)]
mod test {
    use super::*;

    impl From<&str> for Particle {
        fn from(input: &str) -> Self {
            parse_particle(input).unwrap().1
        }
    }

    #[test]
    fn parse() {
        assert_eq!(
            parse_particle("p=<1348,4231,-3011>, v=<-52,-158,114>, a=<-1,-3,2>"),
            Ok((
                "",
                Particle {
                    a: Coord { x: -1, y: -3, z: 2 },
                    v: Coord {
                        x: -52,
                        y: -158,
                        z: 114,
                    },
                    p: Coord {
                        x: 1348,
                        y: 4231,
                        z: -3011,
                    },
                    exploded: false,
                }
            ))
        );
    }
}
