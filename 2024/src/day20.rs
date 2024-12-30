use std::{cmp::min, collections::BinaryHeap, usize};

use aoc::*;

#[derive(Debug)]
struct Node {
    dist: usize,
    x: usize,
    y: usize,
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.dist.partial_cmp(&self.dist)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist && self.x == other.x && self.y == other.y
    }
}

pub struct Racetrack {
    width: usize,
    height: usize,
    shortest_path: usize,
    from_start: Vec<Vec<usize>>,
    from_end: Vec<Vec<usize>>,
}

#[aoc_generator(day20)]
fn generate(input: &str) -> Racetrack {
    let race = Grid::from(input);
    let mut from_start = Grid::of(race.width(), race.height(), usize::MAX);
    let mut from_end = Grid::of(race.width(), race.height(), usize::MAX);
    let start = race.index_of('S').unwrap();
    fill(&start, &race, &mut from_start);
    fill(&race.index_of('E').unwrap(), &race, &mut from_end);
    let shortest_path = from_end.get(start.x, start.y).unwrap();
    Racetrack {
        width: race.width(),
        height: race.height(),
        shortest_path,
        from_start: from_start.into_inner(),
        from_end: from_end.into_inner(),
    }
}

// #[aoc(day20, part1)]
// pub fn part1(input: &Racetrack) -> usize {
//     let mut summary = HashMap::new();
//     let answer = race
//         .points()
//         .filter_map(|p| find_cheats(p, shortest_path, &from_start, &from_end, &mut summary))
//         .sum();
//     let mut keys = summary.keys().collect::<Vec<&usize>>();
//     keys.sort();
//     for &k in keys {
//         debug!("{}: {}", summary.get(&k).unwrap(), k);
//     }
//     answer
// }

// fn find_cheats(
//     p: Point2D<char>,
//     shortest_path: usize,
//     from_start: &Grid<usize>,
//     from_end: &Grid<usize>,
//     summary: &mut HashMap<usize, usize>,
// ) -> Option<usize> {
//     let mut good_cheats = 0;
//     if p.get() != '#' {
//         return None;
//     }
//     let start = min_dist(&p, from_start)?;
//     let end = min_dist(&p, from_end)?;
//     let dist = start + 2 + end;
//     let diff = shortest_path.checked_sub(dist).unwrap_or(0);
//     if diff >= 100 {
//         good_cheats += 1;
//         *summary.entry(diff).or_insert(0) += 1;
//         if diff > 50 {
//             debug!("{}: {} + 2 + {} = {}", p, start, end, diff);
//         }
//     }
//     for n in p.adjacent() {
//         if n.get() != '#' {
//             continue;
//         }
//         let next_start = min_dist(&p, from_start)?;
//         let next_end = min_dist(&p, from_end)?;
//         let next_dist = next_start + 2 + next_end;

//         let next_start = min(start, next_start);
//         let next_end = min(end, next_end);
//         let dist2 = next_start + 3 + next_end;
//         if next_dist <= dist2 {
//             continue;
//         }
//         if let Some(diff2) = shortest_path.checked_sub(dist2) {
//             if diff2 > diff && diff2 >= 100 {
//                 good_cheats += 1;
//                 *summary.entry(diff2).or_insert(0) += 1;
//                 if diff2 > 50 {
//                     debug!("{} + 2 + {} = {}: {}", start, end, diff2, n);
//                 }
//             }
//         }
//     }
//     Some(good_cheats)
// }

// fn min_dist(cell: &Point2D<char>, dist: &Grid<usize>) -> Option<usize> {
//     cell.adjacent()
//         .into_iter()
//         .filter(|c| c.get() != '#')
//         .map(|c| dist.get(c.x, c.y).unwrap())
//         .min()
// }

fn fill(start: &Point2D<char>, race: &Grid<char>, dist: &mut Grid<usize>) {
    let mut stack = BinaryHeap::new();
    stack.push(Node {
        dist: 0,
        x: start.x,
        y: start.y,
    });
    while let Some(node) = stack.pop() {
        dist.set(node.x, node.y, node.dist);
        let next = node.dist + 1;
        for n in race.at(node.x, node.y).unwrap().adjacent() {
            if n.get() != '#' && next < dist.get(n.x, n.y).unwrap() {
                stack.push(Node {
                    dist: next,
                    x: n.x,
                    y: n.y,
                });
            }
        }
    }
}

#[aoc(day20, part2)]
pub fn part2(input: &Racetrack) -> usize {
    let mut cheats = 0;
    for y0 in 1..input.height - 1 {
        let start0 = &input.from_start[y0];
        let end0 = &input.from_end[y0];
        for x0 in 1..input.width - 1 {
            let s0 = start0[x0];
            let e0 = end0[x0];
            if s0 == usize::MAX {
                continue;
            }
            for y1 in y0..input.height - 1 {
                let start1 = &input.from_start[y1];
                let end1 = &input.from_end[y1];
                for x1 in 1..input.width - 1 {
                    if y0 == y1 && x0 >= x1 {
                        continue;
                    }
                    let s1 = start1[x1];
                    let e1 = end1[x1];
                    if s1 == usize::MAX {
                        continue;
                    }
                    let dist = x0.abs_diff(x1) + y0.abs_diff(y1);
                    if dist > 20 {
                        continue;
                    }
                    let shortcut = min(s0 + e1, s1 + e0) + dist;
                    if input.shortest_path.checked_sub(shortcut).unwrap_or(0) >= 100 {
                        cheats += 1;
                    }
                }
            }
        }
    }
    cheats
}
