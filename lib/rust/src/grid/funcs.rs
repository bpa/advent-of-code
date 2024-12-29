use std::cmp::max;

use crate::CellType;

pub fn manhattan_distance(x0: usize, y0: usize, x1: usize, y1: usize) -> usize {
    x1.abs_diff(x0) + y1.abs_diff(y0)
}

pub fn manhattan_distance_with_corners(x0: usize, y0: usize, x1: usize, y1: usize) -> usize {
    max(x1.abs_diff(x0), y1.abs_diff(y0))
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
