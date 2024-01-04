use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    hash::Hash,
    rc::Weak,
};

use crate::CellType;

use super::{a_star::walk_path, point2d::Point2D};

pub struct GridData<T> {
    pub(crate) data: RefCell<Vec<Vec<T>>>,
    pub width: usize,
    pub height: usize,
    pub(crate) dist: fn(x0: usize, y0: usize, x1: usize, y1: usize) -> usize,
    pub(crate) cost: fn(x0: usize, y0: usize, v0: T, x1: usize, y1: usize, v1: T) -> usize,
    pub(crate) neighbors: &'static [(isize, isize)],
}

impl<T: Copy + Eq + Hash> GridData<T> {
    // Find a point containing each item
    pub(crate) fn index<I: IntoIterator<Item = T>>(
        &self,
        items: I,
        data: &Weak<GridData<T>>,
    ) -> HashMap<T, Option<Point2D<T>>> {
        let mut map = HashMap::new();
        for i in items {
            map.insert(i, None);
        }
        let grid = self.data.borrow();
        for (y, row) in grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Entry::Occupied(mut e) = map.entry(*cell) {
                    e.insert(Some(Point2D {
                        x,
                        y,
                        grid: Weak::clone(data),
                    }));
                }
            }
        }
        map
    }
}

impl<T: PartialEq> GridData<T> {
    pub(crate) fn index_of(&self, value: T, grid: Weak<GridData<T>>) -> Option<Point2D<T>> {
        for (y, row) in (*self.data.borrow()).iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                if *val == value {
                    return Some(Point2D { x, y, grid });
                }
            }
        }
        None
    }
}

impl<T: Copy + PartialEq> GridData<T> {
    pub(crate) fn shortest_path(
        &self,
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
        classifier: &dyn Fn(T) -> CellType,
        data: &Weak<GridData<T>>,
    ) -> Option<Vec<Point2D<T>>> {
        self.a_star(x0, y0, classifier, &|_, path, next| {
            if next.x == x1 && next.y == y1 {
                Some(walk_path(x1, y1, path, data))
            } else {
                None
            }
        })
        .next()
    }

    pub(crate) fn reachable_points(
        &self,
        x: usize,
        y: usize,
        classifier: &dyn Fn(T) -> CellType,
        data: &Weak<GridData<T>>,
    ) -> Vec<(Point2D<T>, usize)> {
        self.a_star(x, y, classifier, &|grid_data, _, node| {
            let key = grid_data[node.y][node.x];
            if classifier(key) == CellType::Occupied {
                Some((
                    Point2D {
                        x: node.x,
                        y: node.y,
                        grid: Weak::clone(data),
                    },
                    node.estimate,
                ))
            } else {
                None
            }
        })
        .collect()
    }
}

impl<T: Copy + Eq + Hash> GridData<T> {
    pub(crate) fn dist_to_reachable(
        &self,
        x: usize,
        y: usize,
        classifier: &dyn Fn(T) -> CellType,
    ) -> HashMap<T, usize> {
        self.a_star(x, y, classifier, &|data, _, node| {
            let key = data[node.y][node.x];
            if classifier(key) == CellType::Occupied {
                Some((key, node.estimate))
            } else {
                None
            }
        })
        .collect()
    }

    pub(crate) fn paths_to_reachable(
        &self,
        x: usize,
        y: usize,
        classifier: &dyn Fn(T) -> CellType,
        data: &Weak<GridData<T>>,
    ) -> HashMap<T, Vec<Point2D<T>>> {
        self.a_star(x, y, classifier, &|grid_data, path, node| {
            let key = grid_data[node.y][node.x];
            if classifier(key) == CellType::Occupied {
                Some((key, walk_path(node.x, node.y, path, data)))
            } else {
                None
            }
        })
        .collect()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{common_grid_classifier, grid::construct::Neighbors, Grid, Point2D};

    #[test]
    fn index_of() {
        let g = Grid::from("123\n123");
        let p = g.index_of(b'2');
        assert!(p.is_some());
        let p = p.unwrap();
        assert_eq!((1, 0), (p.x, p.y));

        assert!(g.index_of(b'p').is_none());
    }

    #[test]
    fn shortest_path() {
        let mut g = Grid::from(
            "\
        a.#c\n\
        .#b.\n\
        ...d",
        );
        let ab = g.i('a').shortest_path(&g.i('b'), &common_grid_classifier);
        assert!(ab.is_some());
        let ab = ab.unwrap();
        assert_eq!(5, ab.len());
        assert_eq!((0, 1), (ab[0].x, ab[0].y));
        assert_eq!((0, 2), (ab[1].x, ab[1].y));
        assert_eq!((1, 2), (ab[2].x, ab[2].y));
        assert_eq!((2, 2), (ab[3].x, ab[3].y));
        assert_eq!((2, 1), (ab[4].x, ab[4].y));

        assert_eq!(
            None,
            g.i('a').shortest_path(&g.i('c'), &common_grid_classifier)
        );
        drop(ab);

        g.set_neighbors(Neighbors::Eight);
        let ab = g.i('a').shortest_path(&g.i('c'), &common_grid_classifier);
        assert!(ab.is_some());
        let ab = ab.unwrap();
        assert_eq!(5, ab.len());
        assert_eq!((0, 1), (ab[0].x, ab[0].y));
        assert_eq!((1, 2), (ab[1].x, ab[1].y));
        assert_eq!((2, 2), (ab[2].x, ab[2].y));
        assert_eq!((3, 1), (ab[3].x, ab[3].y));
        assert_eq!((3, 0), (ab[4].x, ab[4].y));
    }

    #[test]
    fn reachable() {
        let mut g = Grid::from(
            "\
        .A#B#C\n\
        #.###.\n\
        D....E",
        );

        let r = g.reachable_dist_from_value('A', &common_grid_classifier);
        println!("{:?}", r);
        assert_eq!(2, r.len());
        assert_eq!(
            HashSet::from([&'D', &'E']),
            r.keys().collect::<HashSet<&char>>()
        );
        drop(r);

        g.set_neighbors(Neighbors::Eight);
        let r = g.reachable_paths_from_cell(5, 0, &common_grid_classifier);
        assert_eq!(3, r.len());
        assert_eq!(
            HashSet::from([&'A', &'D', &'E']),
            r.keys().collect::<HashSet<&char>>()
        );
    }

    impl<T: PartialEq> Grid<T> {
        fn i(&self, value: T) -> Point2D<T> {
            self.index_of(value).unwrap()
        }
    }
}
