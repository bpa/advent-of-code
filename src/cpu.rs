pub struct Intcode<'a> {
    memory: Vec<isize>,
    instruction_pointer: usize,
    relative_base: usize,
    input: &'a mut dyn Iterator<Item = isize>,
}

impl<'a> Intcode<'a> {
    pub fn new(memory: Vec<isize>, input: &'a mut dyn Iterator<Item = isize>) -> Self {
        Intcode {
            memory: memory,
            instruction_pointer: 0usize,
            relative_base: 0usize,
            input: input,
        }
    }

    fn addr(&mut self, register: usize, op: isize) -> usize {
        let place = match register {
            1 => 100,
            2 => 1000,
            3 => 10000,
            _ => 0,
        };

        let mode = (op / place) % 10;
        let addr = match mode {
            0 => self.memory[self.instruction_pointer + register] as usize,
            1 => self.instruction_pointer + register,
            2 => {
                (self.relative_base as isize + self.memory[self.instruction_pointer + register])
                    as usize
            }
            _ => panic!("Unknown addressing mode: {}", mode),
        };
        if addr >= self.memory.len() {
            self.memory.resize(addr + 1, 0);
        }
        addr
    }

    fn value(&mut self, register: usize, op: isize) -> isize {
        let addr = self.addr(register, op);
        self.memory[addr]
    }
}

impl<'a> Iterator for Intcode<'a> {
    type Item = isize;
    fn next(&mut self) -> Option<isize> {
        loop {
            let op = self.memory[self.instruction_pointer];
            let opcode = op % 100;

            let i = self.instruction_pointer;

            macro_rules! a {
                ( $register:literal ) => {
                    self.addr($register, op)
                };
            }
            macro_rules! v {
                ( $register:literal ) => {
                    self.value($register, op)
                };
            }
            if opcode == 4 {
                let out = v!(1);
                self.instruction_pointer = i + 2;
                return Some(out);
            }

            self.instruction_pointer = match opcode {
                1 => {
                    let loc = a!(3);
                    self.memory[loc] = v!(1) + v!(2);
                    i + 4
                }
                2 => {
                    let loc = a!(3);
                    self.memory[loc] = v!(1) * v!(2);
                    i + 4
                }
                3 => {
                    let loc = a!(1);
                    self.memory[loc] = self.input.next().expect("Missing input");
                    i + 2
                }
                5 => match v!(1) {
                    0 => i + 3,
                    _ => v!(2) as usize,
                },
                6 => match v!(1) {
                    0 => v!(2) as usize,
                    _ => i + 3,
                },
                7 => {
                    let loc = a!(3);
                    match v!(1) < v!(2) {
                        true => self.memory[loc] = 1,
                        false => self.memory[loc] = 0,
                    };
                    i + 4
                }
                8 => {
                    let loc = a!(3);
                    match v!(1) == v!(2) {
                        true => self.memory[loc] = 1,
                        false => self.memory[loc] = 0,
                    };
                    i + 4
                }
                9 => {
                    self.relative_base = (self.relative_base as isize + v!(1)) as usize;
                    i + 2
                }
                99 => return None,
                _ => panic!("Unsupported instruction {}", opcode),
            }
        }
    }
}
