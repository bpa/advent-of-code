use std::num::ParseIntError;

#[aoc_generator(day5)]
fn read_memory(input: &str) -> Result<Vec<isize>, ParseIntError> {
    input.split(',').map(|num| num.parse::<isize>()).collect()
}

#[aoc(day5, part1)]
pub fn run_12_2(m: &Vec<isize>) -> isize {
    let code = m.clone();

    run(code, vec![1])
}

//This is me abusing macros to learn the ins and outs
macro_rules! init_registers {
    ($m:ident) => {
        let mut i: usize = 0;
        let mut opcode: usize;
        let mut mode = Vec::with_capacity(3);
        macro_rules! i {
            () => {
                i
            };
        }
        macro_rules! op {
            () => {{
                opcode = $m[i] as usize % 100;
                let mut op: usize = $m[i] as usize / 100;
                mode.clear();
                for _ in 0..3 {
                    mode.push(op % 10 == 1);
                    op = op / 10;
                }
                opcode
            }};
        }
        macro_rules! get {
            ( $register:literal ) => {
                match mode[$register - 1] {
                    true => i + $register,
                    false => $m[i + $register] as usize,
                }
            };
        }
        macro_rules! opcode2 {
            ( $action2:expr ) => {{
                let a = get!(1);
                macro_rules! addr {
                    () => {
                        $m[a]
                    };
                }
                $action2;
                i = i + 2;
            }};
        }
        macro_rules! opcode4 {
            ( $action4:expr ) => {{
                let n = get!(1);
                let v = get!(2);
                let a = get!(3);
                macro_rules! noun {
                    () => {
                        $m[n]
                    };
                }
                macro_rules! verb {
                    () => {
                        $m[v]
                    };
                }
                macro_rules! addr {
                    () => {
                        $m[a]
                    };
                }
                $action4;
                i = i + 4;
            }};
        }
    };
}

fn run(mut m: Vec<isize>, input_data: Vec<isize>) -> isize {
    let mut input = input_data.iter();
    init_registers!(m);
    loop {
        match op!() {
            1 => opcode4!(addr!() = noun!() + verb!()),
            2 => opcode4!(addr!() = noun!() * verb!()),
            3 => opcode2!(addr!() = *input.next().expect("Ran out of input")),
            4 => opcode2!(println!("{}: {}", i!(), addr!())),
            99 => break,
            _ => panic!("Invalid instruction found"),
        }
    }

    m[0]
}
