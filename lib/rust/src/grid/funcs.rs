use std::cmp::max;

use crate::CellType;

pub fn manhattan_distance(x0: usize, y0: usize, x1: usize, y1: usize) -> usize {
    let l = if x0 > x1 { x0 - x1 } else { x1 - x0 };
    let r: usize = if y0 > y1 { y0 - y1 } else { y1 - y0 };
    l + r
}

pub fn manhattan_distance_with_corners(x0: usize, y0: usize, x1: usize, y1: usize) -> usize {
    let l = if x0 > x1 { x0 - x1 } else { x1 - x0 };
    let r: usize = if y0 > y1 { y0 - y1 } else { y1 - y0 };
    max(l, r)
}

pub fn equal_cost<T>(_: usize, _: usize, _: T, _: usize, _: usize, _: T) -> usize {
    1
}

pub fn common_grid_classifier(value: char) -> CellType {
    match value {
        '#' => CellType::Wall,
        '.' => CellType::Empty,
        _ => CellType::Occupied,
    }
}
