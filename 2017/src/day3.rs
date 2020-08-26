use std::cmp::max;
use std::convert::From;

#[aoc_generator(day3)]
pub fn to_usize(input: &str) -> isize {
    input.parse::<isize>().unwrap()
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Self {
        Coord { x, y }
    }

    fn neighbors(&self) -> Vec<Self> {
        vec![
            Coord::new(self.x - 1, self.y - 1),
            Coord::new(self.x - 1, self.y),
            Coord::new(self.x - 1, self.y + 1),
            Coord::new(self.x, self.y - 1),
            Coord::new(self.x, self.y + 1),
            Coord::new(self.x + 1, self.y - 1),
            Coord::new(self.x + 1, self.y),
            Coord::new(self.x + 1, self.y + 1),
        ]
    }
}

impl From<usize> for Coord {
    fn from(input: usize) -> Self {
        if input < 2 {
            return Coord { x: 0, y: 0 };
        }

        let layer = (((input as f64).sqrt() - 1.0) / 2.0).ceil() as isize;
        let offset = 2 * layer;
        let ord = input as isize - (offset - 1).pow(2) - 1;
        let dist = ord % offset;
        let quadrant = ord / offset;
        let coord2 = dist - layer + 1;
        match quadrant {
            0 => Coord::new(layer, coord2),
            1 => Coord::new(-coord2, layer),
            2 => Coord::new(-layer, -coord2),
            _ => Coord::new(coord2, -layer),
        }
    }
}

impl From<Coord> for usize {
    fn from(coord: Coord) -> Self {
        let layer = max(coord.x.abs(), coord.y.abs());
        let quadrant_size = layer * 2;
        let offset = match 1 {
            _ if coord.x == layer && coord.y > -layer => layer + coord.y,
            _ if coord.y == layer => quadrant_size + layer - coord.x,
            _ if coord.x == -layer => 2 * quadrant_size + layer - coord.y,
            _ => 3 * quadrant_size + layer + coord.x,
        };
        ((quadrant_size - 1).pow(2) + offset) as usize
    }
}

#[aoc(day3, part1)]
pub fn steps_to_center(input: &isize) -> isize {
    let coord = Coord::from(*input as usize);
    coord.x.abs() + coord.y.abs()
}

#[aoc(day3, part2)]
pub fn spiral_sum(input: &isize) -> usize {
    let mut values = vec![0; *input as usize];
    let mut sum = 1;
    let mut ind = 1;
    while sum < *input as usize {
        values[ind] = sum;
        ind = ind + 1;
        sum = Coord::from(ind)
            .neighbors()
            .iter()
            .map(|c| values[usize::from(*c)])
            .sum();
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples() {
        let expected: Vec<Vec<isize>> = vec![
            vec![1],
            vec![2, 4, 6, 8],
            vec![3, 5, 7, 9, 11, 15, 19, 23],
            vec![10, 12, 14, 16, 18, 20, 22, 24],
        ];
        for (distance, indexes) in expected.iter().enumerate() {
            for index in indexes {
                assert_eq!(
                    steps_to_center(&index),
                    distance as isize,
                    "[{}] takes {} steps",
                    index,
                    distance
                );
            }
        }
    }

    #[test]
    fn test_from() {
        let coords = vec![
            Coord::new(0, 0), //The problem is 1 based, ignore this one
            Coord::new(0, 0),
            Coord::new(1, 0),
            Coord::new(1, 1),
            Coord::new(0, 1),
            Coord::new(-1, 1),
            Coord::new(-1, 0),
            Coord::new(-1, -1),
            Coord::new(0, -1),
            Coord::new(1, -1),
            Coord::new(2, -1),
            Coord::new(2, 0),
            Coord::new(2, 1),
            Coord::new(2, 2),
            Coord::new(1, 2),
            Coord::new(0, 2),
            Coord::new(-1, 2),
            Coord::new(-2, 2),
            Coord::new(-2, 1),
            Coord::new(-2, 0),
            Coord::new(-2, -1),
            Coord::new(-2, -2),
            Coord::new(-1, -2),
            Coord::new(0, -2),
            Coord::new(1, -2),
            Coord::new(2, -2),
            Coord::new(3, -2),
            Coord::new(3, -1),
        ];
        for i in 1..coords.len() {
            assert_eq!(usize::from(coords[i]), i, "{:?} => {}", &coords[i], i);
            assert_eq!(coords[i], Coord::from(i), "{} => {:?}", i, &coords[i]);
        }
    }
}
