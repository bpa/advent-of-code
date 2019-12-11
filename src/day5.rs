use std::num::ParseIntError;

#[aoc_generator(day5)]
fn read_memory(input: &str) -> Result<Vec<isize>, ParseIntError> {
    input.split(',').map(|num| num.parse::<isize>()).collect()
}

#[aoc(day5, part1)]
pub fn run_12_2(m: &mut [isize]) -> isize {
    m[1] = 12;
    m[2] = 2;

    run(m)
}

macro_rules! instruction {
    ( $m:ident, $i:ident, | $( $v:pat ),* | $code:block ) => {{
        $(
            let $v = $m[$i];
            $i = $i + 1;
        )*
        $code
    }};
    ( $m:ident, $i:ident, | $( $v:pat ),* | $code:stmt ) => { instruction!($m, $i, |$($v),*| { $code })};
    ( $m:ident, $i:ident, | $( $v:pat ),* | $code:expr ) => { instruction!($m, $i, |$($v),*| { $code })};
}

macro_rules! instruction_set {
    ($m:ident $($op:pat => $( $code:tt )*,)*) => {
        let mut i: usize = 0;
        while i < $m.len() {
            match $m[i] {
                $( $op => instruction!($m, i, $($code)*) ),*
            }
        }
    };
}

fn run(m: &mut [isize]) -> isize {
    instruction_set! { m
            1 => | noun, verb, addr| { m[addr] = m[noun] + m[verb] },
            2 => | noun, verb, addr| m[addr] = m[noun] * m[verb],
            99 => break,
            _ =>  panic!("Invalid instruction found"),
    }

    m[0]
}
