use std::{cell::Ref, rc::Rc};

use crate::GridData;

use super::{point2d::Point2D, Grid};

impl<T: Copy> Grid<T> {
    pub fn range(&self, x0: usize, y0: usize, x1: usize, y1: usize) -> GridIter<T> {
        GridIter {
            data: self.data.data.borrow(),
            x0,
            x1: x1 + 1,
            y1: y1 + 1,
            x: x0,
            y: y0,
        }
    }

    pub fn points(&self) -> PointIter<T> {
        PointIter {
            data: Rc::clone(&self.data),
            x0: 0,
            x1: self.data.width,
            y1: self.data.height,
            x: 0,
            y: 0,
        }
    }

    pub fn points_in_range(&self, x0: usize, y0: usize, x1: usize, y1: usize) -> PointIter<T> {
        PointIter {
            data: Rc::clone(&self.data),
            x0,
            x1: x1 + 1,
            y1: y1 + 1,
            x: x0,
            y: y0,
        }
    }
}

impl<'a, T: Copy> IntoIterator for &'a Grid<T> {
    type Item = T;
    type IntoIter = GridIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        GridIter {
            data: self.data.data.borrow(),
            x0: 0,
            x1: self.data.width,
            y1: self.data.height,
            x: 0,
            y: 0,
        }
    }
}

pub struct GridIter<'a, T> {
    data: Ref<'a, Vec<Vec<T>>>,
    x0: usize,
    x1: usize,
    y1: usize,
    x: usize,
    y: usize,
}

impl<'a, T: Copy> Iterator for GridIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.x1 {
            self.x = self.x0;
            self.y += 1;
        }
        if self.y >= self.y1 {
            return None;
        }
        let v = &self.data[self.y][self.x];
        self.x += 1;
        Some(*v)
    }
}

pub struct PointIter<T> {
    data: Rc<GridData<T>>,
    x0: usize,
    x1: usize,
    y1: usize,
    x: usize,
    y: usize,
}

impl<T: Copy> Iterator for PointIter<T> {
    type Item = Point2D<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.x1 {
            self.x = self.x0;
            self.y += 1;
        }
        if self.y >= self.y1 {
            return None;
        }
        let x = self.x;
        let y = self.y;
        self.x += 1;
        Some(Point2D {
            x,
            y,
            grid: Rc::downgrade(&self.data),
        })
    }
}

#[cfg(test)]
mod test {
    use super::super::Grid;

    #[test]
    fn iter() {
        let g = Grid::of(4, 4, 1);
        assert_eq!(16, g.into_iter().sum());
        assert_eq!(4, g.range(1, 1, 2, 2).sum());
        assert_eq!(16, g.points().map(|p| p.get()).sum());
        assert_eq!(4, g.points_in_range(1, 1, 2, 2).map(|p| p.get()).sum());
    }
}
