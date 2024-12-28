use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    usize,
};

use aoc::*;

#[derive(Debug)]
struct Visited {
    cost: usize,
    parents: Vec<(usize, usize, usize)>,
}

#[derive(Debug)]
struct Path {
    v: char,
    n: [Option<Visited>; 4],
}

#[derive(Debug)]
struct Node {
    x: usize,
    y: usize,
    dir: usize,
    cost: usize,
    estimate: usize,
    parent: Option<(usize, usize, usize)>,
}

impl Node {
    fn new(
        node: Option<&Node>,
        x: usize,
        y: usize,
        dir: usize,
        cost: usize,
        x1: usize,
        y1: usize,
    ) -> Self {
        let g = estimate(x, y, dir, x1, y1);
        let parent = node.and_then(|n| Some((n.x, n.y, n.dir)));
        Self {
            x,
            y,
            dir,
            cost,
            estimate: cost + g,
            parent,
        }
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.estimate.cmp(&self.estimate)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.estimate.cmp(&self.estimate))
    }
}

fn estimate(x0: usize, y0: usize, dir: usize, x1: usize, y1: usize) -> usize {
    let base = x0.abs_diff(x1) + y0.abs_diff(y1);
    let dx = x0.cmp(&x1);
    let dy = y0.cmp(&y1);
    let r = base
        + match (dx, dy, dir) {
            (Ordering::Equal, Ordering::Equal, _) => 0,
            (Ordering::Less, _, 4) => 2000,
            (_, Ordering::Less, 0) => 2000,
            (Ordering::Greater, _, 1) => 2000,
            (_, Ordering::Greater, 2) => 2000,
            (_, _, _) => 1000,
        };
    r
}

fn find(needle: char, haystack: &Grid<Path>) -> Option<Point2D<Path>> {
    for (y, row) in haystack.data().iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if value.v == needle {
                return haystack.at(x, y);
            }
        }
    }
    None
}

// TODO: Fix this
// #[aoc(day16, part1)]
// pub fn part1(input: &str) -> usize {
//     let g = Grid::parse_str(input, |c| Path {
//         v: char::from(c),
//         n: [const { None }; 4],
//     });
//     let s = find('S', &g).unwrap();
//     let e = find('E', &g).unwrap();
//     let mut queue = BinaryHeap::with_capacity(g.width() * g.height());
//     queue.push(Node::new(None, s, 1, 0, &e));
//     while let Some(node) = queue.pop() {
//         {
//             let mut data = g.data_mut();
//             let p = &mut data[node.p.y][node.p.x];
//             p.n[node.dir] = Some(Visited {
//                 cost: 0,
//                 parent: None,
//             });
//             if p.v == 'E' {
//                 return node.cost;
//             }
//             let right_turn = (node.dir + 1) % 4;
//             if p.n[right_turn].is_none() {
//                 queue.push(Node::new(
//                     None,
//                     node.p.clone(),
//                     right_turn,
//                     node.cost + 1000,
//                     &e,
//                 ))
//             }
//             let left_turn = (node.dir + 3) % 4;
//             if p.n[left_turn].is_none() {
//                 queue.push(Node::new(
//                     None,
//                     node.p.clone(),
//                     left_turn,
//                     node.cost + 1000,
//                     &e,
//                 ))
//             }
//         }
//         let (x, y) = CARDINAL_4[node.dir];
//         if let Some(next) = node.p.neighbor(x, y) {
//             let neighbor = &g.data()[next.y][next.x];
//             if neighbor.v != '#' && neighbor.n[node.dir].is_none() {
//                 queue.push(Node::new(None, next, node.dir, node.cost + 1, &e));
//             }
//         }
//     }
//     usize::MAX
// }

#[aoc(day16, part2)]
pub fn part2(input: &str) -> usize {
    let g = Grid::parse_str(input, |c| Path {
        v: char::from(c),
        n: [const { None }; 4],
    });
    let mut shortest_path = usize::MAX;
    let mut display = Grid::of(g.width(), g.height(), '#');
    let mut queue = BinaryHeap::with_capacity(g.width() * g.height());
    let (end_x, end_y) = {
        let s = find('S', &g).unwrap();
        let e = find('E', &g).unwrap();
        queue.push(Node::new(None, s.x, s.y, 1, 0, e.x, e.y));
        (e.x, e.y)
    };
    let mut data = g.data_mut();
    while let Some(node) = queue.pop() {
        if node.estimate > shortest_path {
            continue;
        }
        let p = &mut data[node.y][node.x];
        if let Some(visited) = &mut p.n[node.dir] {
            if let Some(parent) = node.parent {
                if node.cost == visited.cost {
                    visited.parents.push(parent);
                }
            }
            continue;
        } else {
            let parents = match node.parent {
                Some(p) => vec![p],
                None => Vec::new(),
            };
            p.n[node.dir] = Some(Visited {
                cost: node.cost,
                parents,
            });
        }
        if p.v == 'E' {
            shortest_path = node.cost;
            continue;
        }
        for d in 0..4 {
            let cost = match node.dir.abs_diff(d) {
                0 => 1,
                1 => 1001,
                3 => 1001,
                _ => continue,
            };
            let x = node.x as isize + CARDINAL_4[d].0;
            let y = node.y as isize + CARDINAL_4[d].1;
            if x >= 0 && y >= 0 {
                let x = x as usize;
                let y = y as usize;
                if let Some(row) = data.get(y) {
                    if let Some(cell) = row.get(x) {
                        if cell.v != '#' {
                            queue.push(Node::new(
                                Some(&node),
                                x,
                                y,
                                d,
                                node.cost + cost,
                                end_x,
                                end_y,
                            ));
                        }
                    }
                }
            }
        }
    }
    let mut queue = Vec::new();
    for (i, visited) in data[end_y][end_x].n.iter().enumerate() {
        if let Some(v) = visited {
            if v.cost == shortest_path {
                queue.push((end_x, end_y, i));
            }
        }
    }
    let mut best_seats = HashSet::new();
    while let Some((x, y, dir)) = queue.pop() {
        best_seats.insert((x, y));
        display.set(x, y, 'O');
        if let Some(visited) = &mut data[y][x].n[dir] {
            queue.append(&mut visited.parents);
        }
    }
    debug!("\n{}", display);
    best_seats.len()
}
