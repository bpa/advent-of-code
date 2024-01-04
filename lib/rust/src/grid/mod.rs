mod a_star;
mod construct;
mod core;
mod funcs;
mod iter;
mod point2d;
mod search;

pub use construct::*;
pub use core::*;
pub use funcs::*;
pub use iter::*;
pub use point2d::*;
use std::{
    cell::{Ref, RefMut},
    fmt::Display,
    rc::Rc,
};

pub struct Grid<T> {
    data: Rc<GridData<T>>,
}

#[derive(PartialEq)]
pub enum CellType {
    Empty,
    Occupied,
    Wall,
}

impl<T> Grid<T> {
    pub fn data(&self) -> Ref<Vec<Vec<T>>> {
        self.data.data.borrow()
    }

    pub fn data_mut(&self) -> RefMut<Vec<Vec<T>>> {
        self.data.data.borrow_mut()
    }
}

impl<T: Clone + Copy> Grid<T> {
    pub fn at(&self, x: usize, y: usize) -> Option<Point2D<T>> {
        if x < self.data.width && y < self.data.height {
            return Some(Point2D {
                x,
                y,
                grid: Rc::downgrade(&self.data),
            });
        }
        None
    }

    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        if x < self.data.width && y < self.data.height {
            return Some(self.data.data.borrow()[y][x]);
        }
        None
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if x < self.data.width && y < self.data.height {
            self.data.data.borrow_mut()[y][x] = value;
        }
    }
}

impl<T: Copy + Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.data.data.borrow().iter() {
            for c in row {
                f.write_fmt(format_args!("{}", c))?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn getter_setters() {
        let mut g = Grid::new(2, 3);
        assert_eq!(Some(0), g.get(0, 0));
        assert_eq!(Some(0), g.get(1, 2));
        assert_eq!(None, g.get(0, 3));
        assert_eq!(None, g.get(2, 0));
        g.set(1, 1, 2);
        assert_eq!(Some(2), g.get(1, 1));
        assert_eq!(2, g.data.data.borrow()[1][1]);
    }

    #[test]
    fn parse() {
        let g = Grid::from("012\n345");
        assert_eq!(Some(b'0'), g.get(0, 0));
        assert_eq!(Some(b'2'), g.get(2, 0));
        assert_eq!(None, g.get(3, 0));
        assert_eq!(Some(b'3'), g.get(0, 1));
        assert_eq!(Some(b'5'), g.get(2, 1));
        assert_eq!(None, g.get(0, 2));
    }

    #[test]
    fn at() {
        let g = Grid::<bool>::new(2, 3);
        assert!(g.at(0, 0).is_some());
        assert!(g.at(1, 0).is_some());
        assert!(g.at(2, 0).is_none());
        assert!(g.at(0, 2).is_some());
        assert!(g.at(1, 2).is_some());
        assert!(g.at(2, 2).is_none());
        assert!(g.at(0, 3).is_none());
    }

    #[test]
    fn display() {
        let mut g = Grid::of(3, 2, '.');
        g.set(0, 0, '#');
        g.set(2, 1, '#');
        assert_eq!("#..\n..#\n", format!("{}", g));
    }
}
