use std::{
    cell::Ref,
    collections::{BinaryHeap, VecDeque},
    fmt::Debug,
    rc::Weak,
};

use crate::{CellType, GridData, Point2D};

type IsGoalFn<'a, T, R> =
    &'a dyn Fn(&Ref<'a, Vec<Vec<T>>>, &Vec<Vec<PathNode>>, &SearchNode) -> Option<R>;

#[derive(Clone, Copy)]
pub(crate) struct PathNode {
    distance: usize,
    parent: (usize, usize),
    visited: bool,
}

impl Debug for PathNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "[{},{:?},{}]",
            self.distance, self.parent, self.visited
        ))?;
        Ok(())
    }
}

#[derive(Eq, PartialEq, Debug)]
pub(crate) struct SearchNode {
    pub(crate) estimate: usize,
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.estimate.cmp(&self.estimate)
    }
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub(crate) struct FillIter<'a, T, R> {
    grid: &'a GridData<T>,
    data: Ref<'a, Vec<Vec<T>>>,
    classifier: &'a dyn Fn(T) -> CellType,
    node: IsGoalFn<'a, T, R>,
    path: Vec<Vec<PathNode>>,
    queue: BinaryHeap<SearchNode>,
}

impl<T> GridData<T> {
    pub(crate) fn a_star<'a, R>(
        &'a self,
        x: usize,
        y: usize,
        classifier: &'a dyn Fn(T) -> CellType,
        node: IsGoalFn<'a, T, R>,
    ) -> FillIter<'a, T, R> {
        let mut queue = BinaryHeap::with_capacity(self.width * self.height);
        let mut path = vec![
            vec![
                PathNode {
                    distance: usize::max_value() / 2,
                    parent: (0, 0),
                    visited: false,
                };
                self.width
            ];
            self.height
        ];
        path[y][x].distance = 0;
        queue.push(SearchNode { estimate: 0, x, y });

        FillIter {
            grid: self,
            data: self.data.borrow(),
            classifier,
            node,
            queue,
            path,
        }
    }
}

impl<'a, T: Copy, R> Iterator for FillIter<'a, T, R> {
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.queue.pop() {
            self.path[next.y][next.x].visited = true;

            if next.estimate > 0 {
                let node = (self.node)(&self.data, &self.path, &next);
                if node.is_some() {
                    return node;
                }

                if (self.classifier)(self.data[next.y][next.x]) != CellType::Empty {
                    continue;
                }
            }

            for (dx, dy) in self.grid.neighbors {
                let nx = next.x as isize + dx;
                let ny = next.y as isize + dy;
                let nx = usize::try_from(nx);
                let ny = usize::try_from(ny);
                if nx.is_err() || ny.is_err() {
                    continue;
                }
                let nx = nx.unwrap();
                let ny = ny.unwrap();
                if nx >= self.grid.width || ny >= self.grid.height {
                    continue;
                }

                if self.path[ny][nx].visited {
                    continue;
                }

                if (self.classifier)(self.data[ny][nx]) == CellType::Wall {
                    continue;
                }

                let g = self.path[next.y][next.x].distance
                    + (self.grid.cost)(
                        next.x,
                        next.y,
                        self.data[next.y][next.x],
                        nx,
                        ny,
                        self.data[ny][nx],
                    );
                let candidate = &mut self.path[ny][nx];
                if g >= candidate.distance {
                    continue;
                }

                candidate.distance = g;
                candidate.parent.0 = next.x;
                candidate.parent.1 = next.y;
                self.queue.push(SearchNode {
                    estimate: g,
                    x: nx,
                    y: ny,
                })
            }
        }
        None
    }
}

pub(crate) fn walk_path<T>(
    mut x: usize,
    mut y: usize,
    path: &[Vec<PathNode>],
    data: &Weak<GridData<T>>,
) -> Vec<Point2D<T>> {
    let distance = path[y][x].distance;
    let mut walk = VecDeque::with_capacity(distance);
    for _ in 0..distance {
        walk.push_front(Point2D {
            x,
            y,
            grid: Weak::clone(data),
        });
        let cell = &path[y][x];
        x = cell.parent.0;
        y = cell.parent.1;
    }
    walk.into()
}
