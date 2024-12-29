use std::{cell::RefCell, cmp::max, rc::Rc};

use crate::manhattan_distance_with_corners;

use super::{
    funcs::{equal_cost, manhattan_distance},
    Grid, GridData,
};

pub const CARDINAL_4: &[(isize, isize)] = &[(0, -1), (1, 0), (0, 1), (-1, 0)];
pub const CARDINAL_8: &[(isize, isize)] = &[
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

impl<T: Copy + Default> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Grid::of(width, height, T::default())
    }
}

impl<T: Clone + Copy + Default> Grid<T> {
    pub fn of(width: usize, height: usize, value: T) -> Self {
        let mut data = Vec::with_capacity(height);
        for _ in 0..height {
            data.push(vec![value; width]);
        }
        Self {
            data: Rc::new(GridData {
                data: RefCell::new(data),
                dist: manhattan_distance,
                cost: equal_cost,
                neighbors: CARDINAL_4,
                width,
                height,
            }),
        }
    }
}

impl<T: Default + From<u8>> From<&[u8]> for Grid<T> {
    fn from(value: &[u8]) -> Self {
        Grid::parse_bytes(value, T::from)
    }
}

impl<T: Default + From<u8>> From<&str> for Grid<T> {
    fn from(value: &str) -> Self {
        Grid::parse_str(value, T::from)
    }
}

impl Grid<u32> {
    pub fn from_digits(value: &str) -> Grid<u32> {
        Grid::parse_bytes(value.as_bytes(), |c: u8| {
            char::from(c).to_digit(10).unwrap()
        })
    }
}

impl<T> Grid<T> {
    pub fn parse_str(value: &str, f: fn(u8) -> T) -> Grid<T> {
        Grid::parse_bytes(value.as_bytes(), f)
    }

    pub fn parse_bytes(value: &[u8], f: fn(u8) -> T) -> Grid<T> {
        let mut data = Vec::new();
        let mut i = 0;
        let mut width = 0;
        while i < value.len() {
            let mut row = Vec::new();
            while i < value.len() {
                let c = value[i];
                if c == b'\r' || c == b'\n' {
                    break;
                }
                row.push(f(c));
                i += 1;
            }
            width = max(width, row.len());
            data.push(row);
            while i < value.len() {
                let c = value[i];
                if c != b'\r' && c != b'\n' {
                    break;
                }
                i += 1;
            }
        }
        let height = data.len();
        Grid {
            data: Rc::new(GridData {
                data: RefCell::new(data),
                dist: manhattan_distance,
                cost: equal_cost,
                neighbors: CARDINAL_4,
                width,
                height,
            }),
        }
    }
}

impl<T: Default> Grid<T> {
    pub fn set_neighbors(&mut self, n: Neighbors) {
        match n {
            Neighbors::Four => Rc::get_mut(&mut self.data).unwrap().neighbors = CARDINAL_4,
            Neighbors::Eight => {
                let grid = Rc::get_mut(&mut self.data).unwrap();
                grid.neighbors = CARDINAL_8;
                if grid.dist == manhattan_distance {
                    grid.dist = manhattan_distance_with_corners;
                }
            }
        }
    }

    pub fn set_distance(&mut self, f: fn(usize, usize, usize, usize) -> usize) {
        Rc::get_mut(&mut self.data).unwrap().dist = f;
    }

    pub fn set_vertex_cost(&mut self, f: fn(usize, usize, T, usize, usize, T) -> usize) {
        Rc::get_mut(&mut self.data).unwrap().cost = f;
    }
}

pub enum Neighbors {
    Four,
    Eight,
}
