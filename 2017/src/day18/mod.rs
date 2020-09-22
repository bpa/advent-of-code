mod instructions;
mod parser;

use self::instructions::{Instruction, RunResult};
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
enum State {
    Running,
    Blocked,
    Halted,
}

#[derive(Debug)]
pub struct Program {
    pub instruction: isize,
    pub registers: [isize; 26],
    pub input: VecDeque<isize>,
    sent: usize,
    state: State,
}

impl Program {
    fn new(id: isize) -> Self {
        Program {
            instruction: 0,
            registers: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, id, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            input: VecDeque::new(),
            sent: 0,
            state: State::Running,
        }
    }
}

#[aoc_generator(day18)]
fn read_instructions(input: &str) -> Vec<Box<dyn Instruction>> {
    parser::parse(input).unwrap().1
}

#[aoc(day18, part1)]
fn first_freq(code: &Vec<Box<dyn Instruction>>) -> isize {
    let mut program = Program::new(0);
    let mut frequency = 0;
    let sentinel = code.len() as isize;
    loop {
        let i = program.instruction as usize;
        match code[i].exec(&mut program) {
            RunResult::Normal => program.instruction += 1,
            RunResult::Jump(v) => program.instruction += v,
            RunResult::Send(v) => {
                frequency = v;
                program.instruction += 1;
            }
            RunResult::Suspend => return frequency,
        }
        if program.instruction < 0 || program.instruction >= sentinel {
            return frequency;
        }
    }
}

#[aoc(day18, part2)]
fn times_sent(code: &Vec<Box<dyn Instruction>>) -> usize {
    let mut programs = [Program::new(0), Program::new(1)];
    let sentinel = code.len() as isize;
    for id in (0..2).cycle() {
        if programs[id].state != State::Running {
            break;
        }

        loop {
            let i = programs[id].instruction as usize;
            match code[i].exec(&mut programs[id]) {
                RunResult::Normal => programs[id].instruction += 1,
                RunResult::Jump(v) => programs[id].instruction += v,
                RunResult::Send(v) => {
                    programs[id].sent += 1;
                    programs[id].instruction += 1;
                    let other = &mut programs[(id + 1) % 2];
                    other.input.push_back(v);
                    if other.state == State::Blocked {
                        other.state = State::Running;
                    }
                }
                RunResult::Suspend => {
                    programs[id].state = State::Blocked;
                    break;
                }
            }

            if programs[id].instruction < 0 || programs[id].instruction >= sentinel {
                programs[id].state = State::Halted;
                break;
            }
        }
    }
    programs[1].sent
}

#[cfg(test)]
mod test {
    use super::instructions::Field::{Register as R, Value as V};
    use super::instructions::*;
    use super::*;

    fn code() -> Vec<Box<dyn Instruction>> {
        vec![
            Box::new(Set { r: R(0), a: V(1) }),
            Box::new(Add { r: R(0), a: V(2) }),
            Box::new(Mul { r: R(0), a: R(0) }),
            Box::new(Mod { r: R(0), a: V(5) }),
            Box::new(Snd { r: R(0) }),
            Box::new(Set { r: R(0), a: V(0) }),
            Box::new(Rcv { r: R(0) }),
            Box::new(Jgz { r: R(0), a: V(-1) }),
            Box::new(Set { r: R(0), a: V(1) }),
            Box::new(Jgz { r: R(0), a: V(-2) }),
        ]
    }

    #[test]
    fn parse() {
        let program = read_instructions(
            "set a 1
        add a 2
        mul a a
        mod a 5
        snd a
        set a 0
        rcv a
        jgz a -1
        set a 1
        jgz a -2",
        );

        assert_eq!(format!("{:?}", program), format!("{:?}", code()));
    }

    #[test]
    fn part1() {
        assert_eq!(4, first_freq(&code()));
    }
}
