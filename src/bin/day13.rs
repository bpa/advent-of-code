extern crate advent_of_code;
extern crate pancurses;

use advent_of_code::cpu::Intcode;
use pancurses::Input;
use std::cell::UnsafeCell;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::thread;
use std::time;

pub fn main() {
    let mut code = read_code().expect("Couldn't open file");
    code[0] = 2;
    let mut score = 0;
    let window = pancurses::initscr();
    window.nodelay(true);
    window.keypad(true);
    pancurses::nl();
    pancurses::noecho();
    pancurses::cbreak();
    pancurses::curs_set(0);
    // let mut joystick = Joystick(&window);
    let objects = UnsafeCell::new((0, 0));
    let mut joystick = NoJoy(&objects);
    let mut game = Intcode::new(code, &mut joystick);
    loop {
        match game.next() {
            Some(x) => {
                let y = game.next().unwrap();
                if x == -1 && y == 0 {
                    score = game.next().unwrap();
                } else {
                    window.mvaddstr(
                        y as i32,
                        x as i32,
                        match game.next().unwrap() {
                            1 => "|",
                            2 => "-",
                            3 => {
                                unsafe { (*objects.get()).0 = x };
                                "="
                            }
                            4 => {
                                unsafe { (*objects.get()).1 = x };
                                "o"
                            }
                            _ => " ",
                        },
                    );
                }
            }
            None => break,
        }
        window.refresh();
    }
    pancurses::endwin();
    println!("Your score was: {}", score);
}

fn read_code() -> Result<Vec<isize>, io::Error> {
    let mut buf = String::new();
    File::open("input/2019/day13.txt")?.read_to_string(&mut buf)?;
    let mem: Vec<isize> = buf
        .trim_end()
        .split(',')
        .filter_map(|num| match num.parse::<isize>() {
            Ok(i) => Some(i),
            Err(_) => None,
        })
        .collect();
    Ok(mem)
}

// I tried playing this, but its way to hard to get to the end
struct Joystick<'a>(&'a pancurses::Window);

impl<'a> Iterator for Joystick<'a> {
    type Item = isize;

    fn next(&mut self) -> Option<isize> {
        thread::sleep(time::Duration::from_millis(300));

        Some(match self.0.getch() {
            Some(Input::KeyLeft) => -1,
            Some(Input::KeyRight) => 1,
            _ => 0,
        })
    }
}

struct NoJoy<'a>(&'a UnsafeCell<(isize, isize)>);

impl<'a> Iterator for NoJoy<'a> {
    type Item = isize;

    fn next(&mut self) -> Option<isize> {
        thread::sleep(time::Duration::from_millis(20));
        unsafe {
            let o = *self.0.get();
            Some(match o.0.cmp(&o.1) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            })
        }
    }
}
