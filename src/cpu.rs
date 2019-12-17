pub struct Intcode<'a> {
    memory: Vec<isize>,
    instruction_pointer: usize,
    input: &'a mut dyn Iterator<Item = isize>,
}

impl<'a> Intcode<'a> {
    pub fn new(memory: Vec<isize>, input: &'a mut dyn Iterator<Item = isize>) -> Self {
        Intcode {
            memory: memory,
            instruction_pointer: 0usize,
            input: input,
        }
    }
}

impl<'a> Iterator for Intcode<'a> {
    type Item = isize;
    fn next(&mut self) -> Option<isize> {
        loop {
            let op = self.memory[self.instruction_pointer];
            let opcode = op % 100;

            let a = |register: usize| {
                let place = match register {
                    1 => 100,
                    2 => 1000,
                    3 => 10000,
                    _ => 0,
                };

                let mode = (op / place) % 10;
                match mode {
                    0 => self.memory[self.instruction_pointer + register] as usize,
                    1 => self.instruction_pointer + register,
                    _ => panic!("Unknown addressing mode: {}", mode),
                }
            };

            let v = |register: usize| self.memory[a(register)];
            let i = self.instruction_pointer;

            if opcode == 4 {
                let out = v(1);
                self.instruction_pointer = i + 2;
                return Some(out);
            }

            self.instruction_pointer = match opcode {
                1 => {
                    let loc = a(3);
                    self.memory[loc] = v(1) + v(2);
                    i + 4
                }
                2 => {
                    let loc = a(3);
                    self.memory[loc] = v(1) * v(2);
                    i + 4
                }
                3 => {
                    let loc = a(1);
                    self.memory[loc] = self.input.next().expect("Missing input");
                    i + 2
                }
                5 => match v(1) {
                    0 => i + 3,
                    _ => v(2) as usize,
                },
                6 => match v(1) {
                    0 => v(2) as usize,
                    _ => i + 3,
                },
                7 => {
                    let loc = a(3);
                    match v(1) < v(2) {
                        true => self.memory[loc] = 1,
                        false => self.memory[loc] = 0,
                    };
                    i + 4
                }
                8 => {
                    let loc = a(3);
                    match v(1) == v(2) {
                        true => self.memory[loc] = 1,
                        false => self.memory[loc] = 0,
                    };
                    i + 4
                }
                99 => return None,
                _ => panic!("Unsupported instruction {}", opcode),
            }
        }
    }
}
