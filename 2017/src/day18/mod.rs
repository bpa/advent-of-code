mod instructions;
mod parser;

use self::instructions::Instruction;

#[derive(Default, Debug)]
pub struct State {
    pub instruction: isize,
    pub registers: [isize; 26],
    pub frequency: isize,
}

#[aoc_generator(day18)]
fn read_instructions(input: &str) -> Vec<Box<dyn Instruction>> {
    parser::parse(input).unwrap().1
}

#[aoc(day18, part1)]
fn first_freq(code: &Vec<Box<dyn Instruction>>) -> isize {
    let mut state = State::default();
    let sentinel = code.len() as isize;
    loop {
        let i = state.instruction as usize;
        if let Some(r) = code[i].exec(&mut state) {
            return r;
        }
        state.instruction += 1;
        if state.instruction < 0 || state.instruction >= sentinel {
            return state.frequency;
        }
    }
}

#[cfg(test)]
mod test {
    use super::instructions::*;
    use super::*;

    fn code() -> Vec<Box<dyn Instruction>> {
        vec![
            Box::new(Set { r: 0, a: 1 }),
            Box::new(Add { r: 0, a: 2 }),
            Box::new(MulR { r: 0, a: 0 }),
            Box::new(Mod { r: 0, a: 5 }),
            Box::new(Snd { r: 0 }),
            Box::new(Set { r: 0, a: 0 }),
            Box::new(Rcv { r: 0 }),
            Box::new(Jgz { r: 0, a: -1 }),
            Box::new(Set { r: 0, a: 1 }),
            Box::new(Jgz { r: 0, a: -2 }),
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
