use cpu::Intcode;
use std::cell::UnsafeCell;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt;
use std::num::ParseIntError;

#[aoc_generator(day11)]
fn read_memory(input: &str) -> Result<Vec<isize>, ParseIntError> {
    input.split(',').map(|num| num.parse::<isize>()).collect()
}

#[aoc(day11, part1)]
pub fn painted_panels(m: &Vec<isize>) -> usize {
    paint(m, false).painted_panels()
}

#[aoc(day11, part2)]
pub fn registration(m: &Vec<isize>) -> Painter {
    paint(m, true)
}

fn paint(m: &Vec<isize>, start_white: bool) -> Painter {
    let painter = UnsafeCell::new(Painter::new());
    if start_white {
        unsafe {
            (*painter.get()).paint(1);
        }
    }
    let mut brain = Intcode::new(m.clone(), unsafe { &mut (*painter.get()) });
    loop {
        unsafe {
            match brain.next() {
                Some(color) => {
                    (*painter.get()).paint(color);
                    (*painter.get()).turn(brain.next().unwrap());
                }
                None => break,
            }
        }
    }
    painter.into_inner()
}

pub struct Painter {
    direction: usize,
    painted: HashMap<(isize, isize), isize>,
    x: isize,
    y: isize,
    x_max: isize,
    x_min: isize,
    y_max: isize,
    y_min: isize,
}

impl Painter {
    fn new() -> Self {
        Painter {
            direction: 0,
            painted: HashMap::new(),
            x: 0,
            y: 0,
            x_max: 0,
            x_min: 0,
            y_max: 0,
            y_min: 0,
        }
    }

    fn paint(&mut self, color: isize) {
        self.painted.insert((self.x, self.y), color);
    }

    fn turn(&mut self, direction: isize) {
        self.direction = match direction {
            0 => (self.direction + 3) % 4,
            1 => (self.direction + 1) % 4,
            _ => panic!("Unknown turn given"),
        };
        match self.direction {
            0 => self.y -= 1,
            1 => self.x += 1,
            2 => self.y += 1,
            3 => self.x -= 1,
            _ => panic!("Mod must be broken"),
        }
        self.x_max = max(self.x_max, self.x);
        self.x_min = min(self.x_min, self.x);
        self.y_max = max(self.y_max, self.y);
        self.y_min = min(self.y_min, self.y);
    }

    fn painted_panels(&self) -> usize {
        self.painted.len()
    }
}

impl Iterator for Painter {
    type Item = isize;

    fn next(&mut self) -> Option<isize> {
        Some(*self.painted.entry((self.x, self.y)).or_default())
    }
}

impl fmt::Display for Painter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = self.x_max - self.x_min;
        let height = self.y_max - self.y_min;
        let mut panels = String::with_capacity((width * height + height) as usize);
        for y in self.y_min..=self.y_max {
            panels.push('\n');
            for x in self.x_min..=self.x_max {
                panels.push(match self.painted.get(&(x, y)) {
                    Some(color) => {
                        if *color == 1 {
                            'X'
                        } else {
                            ' '
                        }
                    }
                    None => ' ',
                })
            }
        }
        write!(f, "{}\n", panels)
    }
}
