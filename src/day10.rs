use num::Integer;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::f64::{INFINITY, NEG_INFINITY};

#[aoc_generator(day10)]
pub fn coordinates(input: &str) -> Vec<(usize, usize)> {
    input
        .split_whitespace()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter_map(move |(x, point)| match point {
                    '#' => Some((x, y)),
                    _ => None,
                })
        })
        .collect()
}

#[aoc(day10, part1)]
pub fn where_to_build(input: &[(usize, usize)]) -> usize {
    max_visibility(input).1
}

#[derive(Debug)]
struct Asteroid {
    x_plane: isize,
    cotangent: f64,
    manhatten_distance: usize,
    x: usize,
    y: usize,
}

#[aoc(day10, part2)]
pub fn zap_em(input: &[(usize, usize)]) -> usize {
    let station = max_visibility(input).0;
    let asteroids = clockwise_from(input, station);

    //I'm not bothering to sweep again since the input has >200 asteroids visible
    let mut zapped: usize = 0;
    let mut last_cotangent: f64 = 0.0;
    for a in asteroids {
        if last_cotangent == a.cotangent {
            continue;
        }
        last_cotangent = a.cotangent;
        zapped += 1;
        if zapped == 200 {
            return a.x * 100 + a.y;
        }
    }
    0
}

fn clockwise_from(input: &[(usize, usize)], station: (usize, usize)) -> Vec<Asteroid> {
    let mut asteroids: Vec<Asteroid> = input
        .iter()
        .map(|point| {
            let x: isize = point.0 as isize - station.0 as isize;
            let y: isize = point.1 as isize - station.1 as isize;
            Asteroid {
                x_plane: match x.cmp(&0) {
                    Ordering::Less => 1,
                    Ordering::Equal => {
                        if y < 0 {
                            -1
                        } else {
                            1
                        }
                    }
                    Ordering::Greater => -1,
                },
                cotangent: if x == 0 {
                    if y == 0 {
                        INFINITY
                    } else {
                        NEG_INFINITY
                    }
                } else {
                    y as f64 / x as f64
                },
                manhatten_distance: x.abs() as usize + y.abs() as usize,
                x: point.0,
                y: point.1,
            }
        })
        .collect();
    asteroids.sort_by(|a, b| {
        a.x_plane.cmp(&b.x_plane).then(
            a.cotangent
                .partial_cmp(&b.cotangent)
                .unwrap()
                .then(a.manhatten_distance.cmp(&b.manhatten_distance)),
        )
    });
    asteroids
}

fn max_visibility(input: &[(usize, usize)]) -> ((usize, usize), usize) {
    let mut visible: Vec<HashSet<(isize, isize)>> = input.iter().map(|_| HashSet::new()).collect();
    for first in 0..input.len() {
        for second in (first + 1)..input.len() {
            let one = input[first];
            let two = input[second];
            let x = one.0 as isize - two.0 as isize;
            let y = one.1 as isize - two.1 as isize;
            let d = x.gcd(&y);
            visible[first].insert((x / d, y / d));
            visible[second].insert((-x / d, -y / d));
        }
    }

    let optimal = visible
        .iter()
        .enumerate()
        .max_by_key(|asteroid| asteroid.1.len())
        .unwrap();
    (input[optimal.0], optimal.1.len())
}

#[cfg(test)]
mod test {
    use super::*;

    impl PartialEq for Asteroid {
        fn eq(&self, other: &Self) -> bool {
            self.x_plane == other.x_plane
                && self.cotangent == other.cotangent
                && self.manhatten_distance == other.manhatten_distance
                && self.x == other.x
                && self.y == other.y
        }
    }
    #[test]
    fn generator() {
        assert_eq!(
            vec!((0, 0), (1, 0), (2, 0), (1, 1), (1, 2), (2, 2)),
            coordinates("###\n.#.\n.##")
        );
    }

    #[test]
    fn clockwise() {
        assert_eq!(
            vec!(
                Asteroid {
                    x_plane: -1,
                    cotangent: NEG_INFINITY,
                    manhatten_distance: 1,
                    x: 1,
                    y: 0,
                },
                Asteroid {
                    x_plane: -1,
                    cotangent: -1.0,
                    manhatten_distance: 2,
                    x: 2,
                    y: 0,
                },
                Asteroid {
                    x_plane: -1,
                    cotangent: 0.0,
                    manhatten_distance: 1,
                    x: 2,
                    y: 1,
                },
                Asteroid {
                    x_plane: -1,
                    cotangent: 1.0,
                    manhatten_distance: 2,
                    x: 2,
                    y: 2,
                },
                Asteroid {
                    x_plane: 1,
                    cotangent: NEG_INFINITY,
                    manhatten_distance: 1,
                    x: 1,
                    y: 2,
                },
                Asteroid {
                    x_plane: 1,
                    cotangent: -1.0,
                    manhatten_distance: 2,
                    x: 0,
                    y: 2,
                },
                Asteroid {
                    x_plane: 1,
                    cotangent: -0.0,
                    manhatten_distance: 1,
                    x: 0,
                    y: 1,
                },
                Asteroid {
                    x_plane: 1,
                    cotangent: 1.0,
                    manhatten_distance: 2,
                    x: 0,
                    y: 0,
                },
                Asteroid {
                    x_plane: 1,
                    cotangent: INFINITY,
                    manhatten_distance: 0,
                    x: 1,
                    y: 1,
                },
            ),
            clockwise_from(
                &[
                    (0, 0),
                    (1, 0),
                    (2, 0),
                    (0, 1),
                    (1, 1),
                    (2, 1),
                    (0, 2),
                    (1, 2),
                    (2, 2)
                ],
                (1, 1)
            )
        );
    }
}
