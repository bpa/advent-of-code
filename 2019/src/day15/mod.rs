mod astar;
mod nearest;
mod point;

use self::astar::*;
use self::nearest::*;
use self::point::*;
use cpu::*;
use std::collections::HashMap;

#[aoc_generator(day15)]
fn generate(input: &str) -> Vec<isize> {
    read_memory(input)
}

#[aoc(day15, part1)]
fn find_distance_to_oxygen(input: &Vec<isize>) -> usize {
    let controller = Input::new(&[]);
    let mut robot = Intcode::new(input.clone(), controller.iter_mut());
    let mut tiles = HashMap::new();
    tiles.insert(Point(0, 0), Status::Open);
    let oxygen = find_oxygen(&controller, &mut robot, &mut tiles);
    best_path_to_oxygen(&oxygen, &controller, &mut robot, &mut tiles).len()
}

#[aoc(day15, part2)]
fn find_max_distance_to_oxygen(input: &Vec<isize>) -> isize {
    let controller = Input::new(&[]);
    let mut robot = Intcode::new(input.clone(), controller.iter_mut());
    let mut tiles = HashMap::new();
    let mut nearest = Nearest::new();
    let mut loc = Point(0, 0);
    let mut oxygen = Point(0, 0);
    tiles.insert(Point(0, 0), Status::Open);
    nearest.populate_adjacent(Point(0, 0), &tiles);

    while let Some(next) = nearest.next_step(&loc, &tiles) {
        controller.go(loc, next);
        let status = match robot.next().unwrap() {
            0 => Status::Wall,
            1 => {
                loc = next;
                nearest.moved_to(next, &tiles);
                Status::Open
            }
            2 => {
                loc = next;
                nearest.moved_to(next, &tiles);
                oxygen = next;
                Status::Oxygen
            }
            _ => panic!("Unknown response from robot"),
        };
        tiles.insert(next, status);
    }
    nearest.search_for_next(&oxygen, &tiles)
}

fn find_oxygen(input: &Input, robot: &mut Intcode, tiles: &mut HashMap<Point, Status>) -> Point {
    let mut nearest = Nearest::new();
    let mut loc = Point(0, 0);
    nearest.populate_adjacent(Point(0, 0), &tiles);

    loop {
        let next = nearest.next_step(&loc, &tiles).unwrap();
        input.go(loc, next);
        let status = match robot.next().unwrap() {
            0 => Status::Wall,
            1 => {
                loc = next;
                nearest.moved_to(next, &tiles);
                Status::Open
            }
            2 => return next,
            _ => panic!("Unknown response from robot"),
        };
        tiles.insert(next, status);
    }
}

impl Input {
    fn go(&self, from: Point, to: Point) {
        self.push(match (to.0 - from.0, to.1 - from.1) {
            (0, 1) => 1,
            (0, -1) => 2,
            (-1, 0) => 3,
            (1, 0) => 4,
            (_, _) => panic!("Developer error"),
        });
    }
}

fn best_path_to_oxygen(
    oxygen: &Point,
    input: &Input,
    robot: &mut Intcode,
    tiles: &mut HashMap<Point, Status>,
) -> Vec<Point> {
    let mut loc = *oxygen;
    let mut routing = AStar::new();
    let mut path = Vec::new();
    let home = Point(0, 0);
    loop {
        routing.shortest_path(&home, oxygen, tiles, &mut path);
        let next = path.iter().find(|p| !tiles.contains_key(p));
        if next == None {
            return path;
        }
        let target = *next.unwrap();
        routing.shortest_path(&loc, &target, tiles, &mut path);
        loc = navigate_to(loc, &mut path, input, robot, tiles);
        routing.shortest_path(&loc, &home, tiles, &mut path);
        loc = navigate_to(loc, &mut path, input, robot, tiles);
    }
}

fn navigate_to(
    mut loc: Point,
    path: &mut Vec<Point>,
    input: &Input,
    robot: &mut Intcode,
    tiles: &mut HashMap<Point, Status>,
) -> Point {
    while !path.is_empty() {
        let next = path.pop().unwrap();
        input.go(loc, next);
        let status = match robot.next().unwrap() {
            0 => Status::Wall,
            1 => Status::Open,
            2 => Status::Oxygen,
            _ => panic!("Unknown response from robot"),
        };
        tiles.insert(next, status);
        if status == Status::Wall {
            return loc;
        }
        loc = next;
    }
    loc
}
