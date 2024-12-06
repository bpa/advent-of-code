use std::collections::{HashMap, HashSet};

use aoc::*;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let mut walked = HashSet::new();
    let room = Grid::from(input);
    let mut guard = room.index_of(b'^').unwrap();
    let mut dir = 0;
    loop {
        walked.insert((guard.x, guard.y));
        let (dx, dy) = CARDINAL_4[dir];
        match guard.neighbor(dx, dy) {
            Some(s) => match s.get() {
                b'#' => dir = (dir + 1) % 4,
                _ => guard = s,
            },
            _ => break,
        };
    }
    walked.len()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let mut walked = HashSet::new();
    let mut placed = 0;
    let room = Grid::from(input);
    let mut guard = room.index_of(b'^').unwrap();
    let mut dir = 0;
    loop {
        walked.insert((guard.x, guard.y));
        let (dx, dy) = CARDINAL_4[dir];
        match guard.neighbor(dx, dy) {
            Some(s) => match s.get() {
                b'#' => dir = (dir + 1) % 4,
                _ => {
                    if !walked.contains(&(s.x, s.y)) {
                        s.set(b'#');
                        if brute_force_loop(guard.clone(), dir) {
                            placed += 1;
                        }
                        s.set(b'.');
                    }
                    guard = s;
                }
            },
            _ => break,
        };
    }
    placed
}

struct Obstacle {
    dirs: [bool; 4],
}

impl Obstacle {
    fn new() -> Self {
        Obstacle { dirs: [false; 4] }
    }

    fn bumped(&mut self, dir: usize) -> bool {
        let r = self.dirs[dir];
        self.dirs[dir] = true;
        r
    }
}

fn brute_force_loop(mut guard: Point2D<u8>, mut dir: usize) -> bool {
    let mut bumped = HashMap::new();
    loop {
        let (dx, dy) = CARDINAL_4[dir];
        match guard.neighbor(dx, dy) {
            Some(s) => match s.get() {
                b'#' => {
                    if bumped
                        .entry((s.x, s.y))
                        .or_insert_with(Obstacle::new)
                        .bumped(dir)
                    {
                        return true;
                    } else {
                        dir = (dir + 1) % 4;
                    }
                }
                _ => guard = s,
            },
            None => return false,
        }
    }
}
