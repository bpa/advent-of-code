use super::*;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;

pub struct AStar {
    next: PriorityQueue<Point, Reverse<isize>>,
    seen: HashMap<Point, Tile>,
}

impl AStar {
    pub fn new() -> Self {
        AStar {
            next: PriorityQueue::with_default_hasher(),
            seen: HashMap::new(),
        }
    }

    pub fn shortest_path(
        &mut self,
        from: &Point,
        to: &Point,
        tiles: &HashMap<Point, Status>,
        path: &mut Vec<Point>,
    ) {
        self.seen.clear();
        self.next.clear();
        self.seen.insert(
            *from,
            Tile {
                g: 0,
                status: None,
                from: None,
            },
        );
        self.next.push(*from, Reverse(0));
        while let Some((p, _)) = self.next.pop() {
            let tile = self.seen.get(&p).unwrap();
            let g = tile.g + 1;
            let (x, y) = (p.0, p.1);
            if self.neighbor(&p, to, g, x, y - 1, &tiles)
                || self.neighbor(&p, to, g, x, y + 1, &tiles)
                || self.neighbor(&p, to, g, x + 1, y, &tiles)
                || self.neighbor(&p, to, g, x - 1, y, &tiles)
            {
                self.trace_path(to, path);
                return;
            }
        }
        panic!("No path found")
    }

    fn neighbor(
        &mut self,
        from: &Point,
        to: &Point,
        g: isize,
        x: isize,
        y: isize,
        tiles: &HashMap<Point, Status>,
    ) -> bool {
        let point = Point(x, y);
        let tile = self.seen.entry(point).or_insert_with(|| Tile {
            g: isize::max_value(),
            status: match tiles.get(&point) {
                Some(s) => Some(*s),
                None => None,
            },
            from: Some(*from),
        });
        if point == *to {
            return true;
        }
        if tile.status != Some(Status::Wall) && g < tile.g {
            tile.from = Some(*from);
            tile.g = g;
            let priority = g + point.distance(&to);
            self.next.push(point, Reverse(priority));
        }
        false
    }

    fn trace_path(&self, from: &Point, points: &mut Vec<Point>) {
        points.clear();
        let mut point = *from;
        while let Some(tile) = self.seen.get(&point) {
            points.push(point);
            point = match tile.from {
                Some(p) => p,
                None => break,
            };
        }
        points.pop();
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Tile {
    g: isize,
    status: Option<Status>,
    from: Option<Point>,
}
