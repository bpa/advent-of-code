use std::u64;

use aoc::*;
use itertools::Itertools;
use nom::character::complete;

//#[aoc_generator(day17)]
//pub fn parse(input: &str) -> &str {
//    input
//}

#[derive(Debug)]
struct Mem {
    a: u64,
    b: u64,
    c: u64,
    out: Vec<u64>,
}

impl Mem {
    fn combo(&self, inst: u64) -> u64 {
        match inst {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => 0,
        }
    }

    fn run(&mut self, code: &[u64]) {
        let mut pos = 0;
        while let Some(op) = code.get(pos) {
            pos = match self.exec(*op, code[pos + 1]) {
                Some(jmp) => jmp,
                None => pos + 2,
            }
        }
    }

    fn exec(&mut self, op: u64, arg: u64) -> Option<usize> {
        match op {
            //adv
            0 => self.a = self.a >> self.combo(arg),
            //bxl
            1 => self.b = self.b ^ arg,
            //bst
            2 => self.b = self.combo(arg) % 8,
            //jnz
            3 => {
                if self.a != 0 {
                    return Some(arg as usize);
                }
            }
            //bxc
            4 => self.b = self.b ^ self.c,
            //out
            5 => self.out.push(self.combo(arg) % 8),
            //bdv
            6 => self.b = self.a >> self.combo(arg),
            //cdv
            7 => self.c = self.a >> self.combo(arg),
            _ => {}
        }
        None
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> String {
    let nums = comb(complete::u64)(input);
    let mut mem = Mem {
        a: nums[0],
        b: nums[1],
        c: nums[2],
        out: Vec::new(),
    };

    mem.run(&nums[3..]);
    mem.out.into_iter().map(|n| n.to_string()).join(",")
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> u64 {
    let nums = comb(complete::u64)(input);
    let mut mem = Mem {
        a: nums[0],
        b: nums[1],
        c: nums[2],
        out: Vec::new(),
    };
    let code = &nums[3..];
    let acc = vec![];
    let base = acc.iter().fold(0, |a, b| (a << 3) | b);
    println!("base {:b}", base);
    for a in 0..8 {
        let display = (a << 3) | base;
        mem.a = (a << 3) | base;
        mem.b = nums[1];
        mem.c = nums[2];
        mem.out.clear();
        mem.run(code);
        println!("A {}|{:b} = {:?}", a, display, mem.out);
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    fn test(a: u64, b: u64, c: u64, code: Vec<u64>) -> Mem {
        let mut mem = Mem {
            a,
            b,
            c,
            out: Vec::new(),
        };
        mem.run(&code);
        mem
    }

    #[test]
    fn test_div() {
        let mem = test(16, 2, 4, vec![0, 5]);
        assert_eq!(4, mem.a);
        let mem = test(0, 0, 9, vec![2, 6]);
        assert_eq!(1, mem.b);
        let mem = test(10, 0, 0, vec![5, 0, 5, 1, 5, 4]);
        assert_eq!(vec![0, 1, 2], mem.out);
        let mem = test(2024, 0, 0, vec![0, 1, 5, 4, 3, 0]);
        assert_eq!(vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0], mem.out);
        assert_eq!(0, mem.a);
        let mem = test(0, 29, 0, vec![1, 7]);
        assert_eq!(26, mem.b);
        let mem = test(0, 2024, 43690, vec![4, 0]);
        assert_eq!(44354, mem.b);
    }
}
