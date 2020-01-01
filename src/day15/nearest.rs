use super::*;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct Nearest {
    adjacent: Vec<Point>,
    path: Vec<Point>,
}

impl Nearest {
    pub fn new() -> Self {
        Nearest {
            adjacent: Vec::new(),
            path: Vec::new(),
        }
    }

    pub fn next_step(&mut self, loc: &Point, tiles: &HashMap<Point, Status>) -> Option<Point> {
        if let Some(t) = self.adjacent.pop() {
            return Some(t);
        }

        if self.path.is_empty() {
            self.search_for_next(loc, tiles);
        }

        self.path.pop()
    }

    pub fn populate_adjacent(&mut self, loc: Point, tiles: &HashMap<Point, Status>) {
        macro_rules! add {
            ($x:expr, $y:expr) => {
                let p = Point($x, $y);
                if !tiles.contains_key(&p) {
                    self.adjacent.push(p);
                }
            };
        }
        let x = loc.0;
        let y = loc.1;
        add!(x + 1, y);
        add!(x - 1, y);
        add!(x, y + 1);
        add!(x, y - 1);
    }

    pub fn moved_to(&mut self, loc: Point, tiles: &HashMap<Point, Status>) {
        self.adjacent.clear();
        self.populate_adjacent(loc, tiles);
    }

    pub fn search_for_next(&mut self, loc: &Point, tiles: &HashMap<Point, Status>) -> isize {
        let mut queue = PriorityQueue::new();
        let mut seen = HashMap::new();
        let mut max: isize = 0;
        queue.push(*loc, Reverse(0));
        let mut target = loop {
            let (next, dist) = match queue.pop() {
                Some(v) => v,
                None => return max,
            };
            max = dist.0;
            let mut look = |x: isize, y: isize| {
                let p = Point(x, y);
                match seen.entry(p) {
                    Entry::Occupied(_) => return None,
                    Entry::Vacant(e) => {
                        if let Some(t) = tiles.get(&p) {
                            e.insert(next);
                            if *t == Status::Open {
                                queue.push(p, Reverse(dist.0 + 1));
                            }
                            return None;
                        }
                        return Some(next);
                    }
                }
            };
            let (x, y) = (next.0, next.1);
            if let Some(p) = look(x + 1, y) {
                break p;
            }
            if let Some(p) = look(x, y - 1) {
                break p;
            }
            if let Some(p) = look(x - 1, y) {
                break p;
            }
            if let Some(p) = look(x, y + 1) {
                break p;
            }
        };
        while let Some(s) = seen.get(&target) {
            self.path.push(target);
            target = *s;
            if s == loc {
                break;
            }
        }
        max
    }
}
