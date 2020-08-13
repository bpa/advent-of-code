#[derive(Debug, Eq, Copy, Clone, Hash, PartialEq)]
pub struct Point(pub isize, pub isize);

impl Point {
    pub fn distance(&self, other: &Point) -> isize {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    Wall,
    Open,
    Oxygen,
}
