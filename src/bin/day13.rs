extern crate advent_of_code;
extern crate pancurses;

use advent_of_code::cpu::Intcode;
use pancurses::Input;
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
    let mut joystick = Joystick(&window);
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
                            3 => "=",
                            4 => "❄",
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

struct Joystick<'a>(&'a pancurses::Window);
struct NoJoy();

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
impl Iterator for NoJoy {
    type Item = isize;

    fn next(&mut self) -> Option<isize> {
        Some(0)
    }
}
