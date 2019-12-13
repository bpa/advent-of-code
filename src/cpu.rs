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
        macro_rules! opcode3 {
            ( $action3:expr ) => {{
                let n = get!(1);
                let a = get!(2);
                macro_rules! noun {
                    () => {
                        $m[n]
                    };
                }
                macro_rules! addr {
                    () => {
                        $m[a]
                    };
                }
                $action3;
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

pub fn run(
    mut m: Vec<isize>,
    input_data: Vec<isize>,
    output: &mut dyn FnMut(usize, isize),
) -> isize {
    let mut input = input_data.iter();
    init_registers!(m);
    loop {
        match op!() {
            1 => opcode4!(addr!() = noun!() + verb!()),
            2 => opcode4!(addr!() = noun!() * verb!()),
            3 => opcode2!(addr!() = *input.next().expect("Ran out of input")),
            4 => opcode2!(output(i!(), addr!())),
            5 => opcode3!(
                i!() = match noun!() {
                    0 => i!() + 3,
                    _ => addr!() as usize,
                }
            ),
            6 => opcode3!(
                i!() = match noun!() {
                    0 => addr!() as usize,
                    _ => i!() + 3,
                }
            ),
            7 => opcode4!(
                addr!() = match noun!() < verb!() {
                    true => 1,
                    false => 0,
                }
            ),
            8 => opcode4!(
                addr!() = match noun!() == verb!() {
                    true => 1,
                    false => 0,
                }
            ),
            99 => break,
            _ => panic!("Invalid instruction found"),
        }
    }

    m[0]
}
