use num::Signed;
use std::{
    fmt::Display,
    ops::{Add, Sub},
};

pub struct Point<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Point<T>
where
    T: Copy + Add + Sub + Signed,
{
    pub fn manhatten_dist(&self, other: Point<T>) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl<T: Display> Display for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn manhatten_dist() {
        assert_eq!(3, Point::new(0, 0).manhatten_dist(Point::new(1, 2)));
        assert_eq!(3, Point::new(0, 0).manhatten_dist(Point::new(1, -2)));
    }

    #[test]
    fn display() {
        assert_eq!("(3, 4)", format!("{}", Point::new(3, 4)))
    }
}
