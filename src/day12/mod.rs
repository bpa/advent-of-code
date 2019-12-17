use std::cell::RefCell;
use std::cmp::Ordering;
mod parser;

use self::parser::coordinates;

#[aoc_generator(day12)]
pub fn moons(input: &str) -> Vec<Coordinate> {
    coordinates(input).unwrap().1
}

#[aoc(day12, part1)]
pub fn tracking(moons: &[Coordinate]) -> isize {
    let mut system: Vec<RefCell<Moon>> =
        moons.iter().map(|c| RefCell::new(Moon::from(c))).collect();
    let moons = system.len();
    for _ in 0..1000 {
        for moon in 0..moons {
            for other in (moon + 1)..moons {
                let a = &system[moon];
                let b = &system[other];
                a.borrow_mut().gravitate_to(&mut b.borrow_mut());
            }
        }
        for moon in system.iter_mut() {
            moon.borrow_mut().move_step();
        }
    }
    system.iter().map(|moon| moon.borrow_mut().energy()).sum()
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coordinate {
    x: isize,
    y: isize,
    z: isize,
}

impl Coordinate {
    fn energy(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

pub struct Moon {
    position: Coordinate,
    velocity: Coordinate,
}

impl From<&Coordinate> for Moon {
    fn from(coord: &Coordinate) -> Self {
        Moon {
            position: *coord,
            velocity: Coordinate { x: 0, y: 0, z: 0 },
        }
    }
}

impl Moon {
    fn gravitate_to(&mut self, other: &mut Moon) {
        macro_rules! attract {
            ( $plane:ident ) => {
                match self.position.$plane.cmp(&other.position.$plane) {
                    Ordering::Less => {
                        self.velocity.$plane += 1;
                        other.velocity.$plane -= 1;
                    }
                    Ordering::Equal => (),
                    Ordering::Greater => {
                        self.velocity.$plane -= 1;
                        other.velocity.$plane += 1;
                    }
                }
            };
        }
        attract!(x);
        attract!(y);
        attract!(z);
    }

    fn move_step(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    fn energy(&self) -> isize {
        self.position.energy() * self.velocity.energy()
    }
}
