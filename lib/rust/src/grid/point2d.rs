use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
    rc::Weak,
};

use crate::CellType;

use super::core::GridData;

pub struct Point2D<T> {
    pub x: usize,
    pub y: usize,
    pub(super) grid: Weak<GridData<T>>,
}

impl<T> Point2D<T> {
    /// Get distance to other point.
    /// Ignores obstacles
    pub fn manhattan_distance(&self, other: &Point2D<T>) -> usize {
        (self.x as isize - other.x as isize).unsigned_abs()
            + (self.y as isize - other.y as isize).unsigned_abs()
    }
}

impl<T> Clone for Point2D<T> {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            grid: self.grid.clone(),
        }
    }
}

impl<T> Eq for Point2D<T> {}

impl<T> PartialEq for Point2D<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T> Hash for Point2D<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl<T> Point2D<T> {
    /// Get point relative to current point
    pub fn neighbor(&self, x: isize, y: isize) -> Option<Point2D<T>> {
        let grid = self.grid.upgrade()?;
        let data = grid.data.borrow();
        let ny = usize::try_from(self.y as isize + y).ok()?;
        let row = data.get(ny)?;
        let nx = usize::try_from(self.x as isize + x).ok()?;
        if nx >= row.len() {
            return None;
        }
        Some(Point2D {
            x: nx,
            y: ny,
            grid: Weak::clone(&self.grid),
        })
    }

    pub fn to_dir(&self, dir: usize) -> Option<Point2D<T>> {
        let grid = self.grid.upgrade()?;
        let data = grid.data.borrow();
        let dir = grid.neighbors[dir];
        let ny = usize::try_from(self.y as isize + dir.1).ok()?;
        let row = data.get(ny)?;
        let nx = usize::try_from(self.x as isize + dir.0).ok()?;
        if nx >= row.len() {
            return None;
        }
        Some(Point2D {
            x: nx,
            y: ny,
            grid: Weak::clone(&self.grid),
        })
    }

    pub fn n(&self) -> Option<Point2D<T>> {
        self.neighbor(0, -1)
    }

    pub fn ne(&self) -> Option<Point2D<T>> {
        self.neighbor(1, -1)
    }

    pub fn e(&self) -> Option<Point2D<T>> {
        self.neighbor(1, 0)
    }

    pub fn se(&self) -> Option<Point2D<T>> {
        self.neighbor(1, 1)
    }

    pub fn s(&self) -> Option<Point2D<T>> {
        self.neighbor(0, 1)
    }

    pub fn sw(&self) -> Option<Point2D<T>> {
        self.neighbor(-1, 1)
    }

    pub fn w(&self) -> Option<Point2D<T>> {
        self.neighbor(-1, 0)
    }

    pub fn nw(&self) -> Option<Point2D<T>> {
        self.neighbor(-1, -1)
    }

    pub fn adjacent(&self) -> Vec<Point2D<T>> {
        self.grid
            .upgrade()
            .unwrap()
            .neighbors
            .iter()
            .filter_map(|(x, y)| self.neighbor(*x, *y))
            .collect()
    }
}

impl<T: Copy> Point2D<T> {
    pub fn get(&self) -> T {
        self.grid.upgrade().unwrap().data.borrow()[self.y][self.x]
    }

    pub fn set(&self, value: T) {
        self.grid.upgrade().unwrap().data.borrow_mut()[self.y][self.x] = value;
    }
}

impl<T: Copy + PartialEq> Point2D<T> {
    pub fn shortest_path(
        &self,
        other: &Point2D<T>,
        classifier: &dyn Fn(T) -> CellType,
    ) -> Option<Vec<Point2D<T>>> {
        self.grid
            .upgrade()
            .unwrap()
            .shortest_path(self.x, self.y, other.x, other.y, classifier, &self.grid)
    }
}

impl<T: Copy + Eq + Hash> Point2D<T> {
    pub fn dist_to_reachable(&self, classifier: &dyn Fn(T) -> CellType) -> HashMap<T, usize> {
        self.grid
            .upgrade()
            .unwrap()
            .dist_to_reachable(self.x, self.y, classifier)
    }

    pub fn paths_to_reachable(
        &self,
        classifier: &dyn Fn(T) -> CellType,
    ) -> HashMap<T, Vec<Point2D<T>>> {
        self.grid
            .upgrade()
            .unwrap()
            .paths_to_reachable(self.x, self.y, classifier, &self.grid)
    }

    pub fn reachable_points(&self, classifier: &dyn Fn(T) -> CellType) -> Vec<(Point2D<T>, usize)> {
        self.grid
            .upgrade()
            .unwrap()
            .reachable_points(self.x, self.y, classifier, &self.grid)
    }
}

impl<T: Display> Display for Point2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(grid) = self.grid.upgrade() {
            let cell = &grid.data.borrow()[self.y][self.x];
            f.write_fmt(format_args!("({}, {})[{}]", self.x, self.y, cell))?;
        } else {
            f.write_fmt(format_args!("({}, {})XXX", self.x, self.y))?;
        }
        Ok(())
    }
}

impl<T: Debug> Debug for Point2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(grid) = self.grid.upgrade() {
            let value = &grid.data.borrow()[self.y][self.x];
            f.debug_struct("Point")
                .field("value", &value)
                .field("x", &self.x)
                .field("y", &self.y)
                .finish()
        } else {
            f.debug_struct("Point")
                .field("x", &self.x)
                .field("y", &self.y)
                .finish()
        }
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use super::*;
    use crate::grid::{construct::Neighbors, Grid};

    #[test]
    fn get() {
        let g = Grid::from_digits("123\n456\n789");
        assert_eq!(1, g.p(0, 0).get());
        assert_eq!(5, g.p(1, 1).get());
        assert_eq!(9, g.p(2, 2).get());
        assert_eq!(5, g.p(0, 0).get() + g.p(0, 1).get());
    }

    #[test]
    fn set() {
        let g = Grid::new(2, 2);
        g.p(1, 1).set(8);
        assert_eq!(8, g.data.data.borrow()[1][1]);
    }

    #[test]
    fn distance() {
        let g: Grid<u8> = Grid::new(3, 3);
        let p1 = g.at(0, 0).unwrap();
        let p2 = g.at(1, 2).unwrap();
        assert_eq!(3, p1.manhattan_distance(&p2));
    }

    #[test]
    fn adjacent() {
        let mut g: Grid<u8> = Grid::new(3, 3);
        assert_eq!(2, g.p(0, 0).adjacent().len());
        assert_eq!(3, g.p(1, 0).adjacent().len());
        assert_eq!(2, g.p(2, 0).adjacent().len());
        assert_eq!(3, g.p(0, 1).adjacent().len());
        assert_eq!(4, g.p(1, 1).adjacent().len());
        assert_eq!(3, g.p(2, 1).adjacent().len());
        assert_eq!(2, g.p(0, 2).adjacent().len());
        assert_eq!(3, g.p(1, 2).adjacent().len());
        assert_eq!(2, g.p(2, 2).adjacent().len());

        g.set_neighbors(Neighbors::Eight);
        assert_eq!(3, g.p(0, 0).adjacent().len());
        assert_eq!(5, g.p(1, 0).adjacent().len());
        assert_eq!(3, g.p(2, 0).adjacent().len());
        assert_eq!(5, g.p(0, 1).adjacent().len());
        assert_eq!(8, g.p(1, 1).adjacent().len());
        assert_eq!(5, g.p(2, 1).adjacent().len());
        assert_eq!(3, g.p(0, 2).adjacent().len());
        assert_eq!(5, g.p(1, 2).adjacent().len());
        assert_eq!(3, g.p(2, 2).adjacent().len());
    }

    #[test]
    fn neighbors() {
        let g: Grid<u8> = Grid::new(3, 3);
        let p = g.p(1, 1);
        assert_eq!(g.at(1, 0), p.n());
        assert_eq!(g.at(2, 0), p.ne());
        assert_eq!(g.at(2, 1), p.e());
        assert_eq!(g.at(2, 2), p.se());
        assert_eq!(g.at(1, 2), p.s());
        assert_eq!(g.at(0, 2), p.sw());
        assert_eq!(g.at(0, 1), p.w());
        assert_eq!(g.at(0, 0), p.nw());

        let g: Grid<u8> = Grid::new(1, 1);
        let p = g.p(0, 0);
        assert_eq!(None, p.n());
        assert_eq!(None, p.ne());
        assert_eq!(None, p.e());
        assert_eq!(None, p.se());
        assert_eq!(None, p.s());
        assert_eq!(None, p.sw());
        assert_eq!(None, p.w());
        assert_eq!(None, p.nw());
    }

    impl<T: Copy> Grid<T> {
        pub(crate) fn p(&self, x: usize, y: usize) -> Point2D<T> {
            Point2D {
                x: x,
                y: y,
                grid: Rc::downgrade(&self.data),
            }
        }
    }
}
