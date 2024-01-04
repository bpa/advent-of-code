use std::hash::Hash;
use std::{collections::HashMap, rc::Rc};

use crate::CellType;

use super::{point2d::Point2D, Grid};

impl<T: Copy + Eq + Hash> Grid<T> {
    pub fn index<I: IntoIterator<Item = T>>(&self, items: I) -> HashMap<T, Option<Point2D<T>>> {
        self.data.index(items, &Rc::downgrade(&self.data))
    }

    pub fn shortest_path(
        &self,
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
        classifier: &dyn Fn(T) -> CellType,
    ) -> Option<Vec<Point2D<T>>> {
        self.data
            .shortest_path(x0, y0, x1, y1, classifier, &Rc::downgrade(&self.data))
    }

    pub fn reachable_paths_from_cell(
        &self,
        x: usize,
        y: usize,
        classifier: &dyn Fn(T) -> CellType,
    ) -> HashMap<T, Vec<Point2D<T>>> {
        self.data
            .paths_to_reachable(x, y, classifier, &Rc::downgrade(&self.data))
    }

    pub fn reachable_paths_from_value(
        &self,
        value: T,
        classifier: &dyn Fn(T) -> CellType,
    ) -> HashMap<T, Vec<Point2D<T>>> {
        match self.data.index_of(value, Rc::downgrade(&self.data)) {
            Some(p) => {
                self.data
                    .paths_to_reachable(p.x, p.y, classifier, &Rc::downgrade(&self.data))
            }
            None => HashMap::new(),
        }
    }

    pub fn reachable_dist_from_cell(
        &self,
        x: usize,
        y: usize,
        classifier: &dyn Fn(T) -> CellType,
    ) -> HashMap<T, usize> {
        self.data.dist_to_reachable(x, y, classifier)
    }

    pub fn reachable_dist_from_value(
        &self,
        value: T,
        classifier: &dyn Fn(T) -> CellType,
    ) -> HashMap<T, usize> {
        match self.data.index_of(value, Rc::downgrade(&self.data)) {
            Some(p) => self.data.dist_to_reachable(p.x, p.y, classifier),
            None => HashMap::new(),
        }
    }
}

impl<T: PartialEq> Grid<T> {
    pub fn index_of(&self, value: T) -> Option<Point2D<T>> {
        self.data.index_of(value, Rc::downgrade(&self.data))
    }
}
#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::Grid;

    #[test]
    fn index() {
        let g = Grid::from_digits("1234\n5678");
        assert_eq!(
            g.index([1, 3, 6, 8, 10]),
            HashMap::from([
                (1, Some(g.p(0, 0))),
                (3, Some(g.p(2, 0))),
                (6, Some(g.p(1, 1))),
                (8, Some(g.p(3, 1))),
                (10, None),
            ])
        );
    }
}
