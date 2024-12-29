use std::collections::HashMap;

use aoc::*;

//#[aoc_generator(day24)]
//pub fn parse(input: &str) -> &str {
//    input
//}

#[derive(Debug)]
struct Gate<'a> {
    a: &'a str,
    b: &'a str,
    op: &'a str,
    r: &'a str,
    deps: usize,
}

#[derive(Default)]
struct Registers<'a> {
    constants: HashMap<&'a str, bool>,
    wanted: HashMap<&'a str, Vec<&'a str>>,
    waiting: HashMap<&'a str, Gate<'a>>,
}

impl<'a> Registers<'a> {
    fn add(&mut self, register: &'a str, value: bool) {
        self.constants.insert(register, value);
        if let Some(wanted) = self.wanted.remove(register) {
            for r in wanted {
                let mut w = self.waiting.remove(r).unwrap();
                w.deps -= 1;
                match w.deps {
                    0 => self.eval(w),
                    _ => {
                        self.waiting.insert(r, w);
                    }
                }
            }
        }
    }

    fn add_gate(&mut self, op: &'a str, a: &'a str, b: &'a str, r: &'a str) {
        let mut gate = Gate {
            a,
            b,
            op,
            r,
            deps: 0,
        };

        if !self.constants.contains_key(a) {
            self.wanted.entry(a).or_insert_with(|| Vec::new()).push(r);
            gate.deps += 1;
        }
        if !self.constants.contains_key(b) {
            self.wanted.entry(b).or_insert_with(|| Vec::new()).push(r);
            gate.deps += 1
        }
        if gate.deps > 0 {
            self.waiting.insert(r, gate);
        } else {
            self.eval(gate);
        }
    }

    fn eval(&mut self, gate: Gate<'a>) {
        if gate.deps == 0 {
            let &a = self.constants.get(gate.a).unwrap();
            let &b = self.constants.get(gate.b).unwrap();
            let val = match gate.op {
                "AND" => a && b,
                "OR" => a || b,
                _ => a != b,
            };
            self.add(gate.r, val);
        }
    }

    fn register(&self, r: char) -> usize {
        let mut z = 0;
        let mut bits = Vec::new();
        while let Some(&b) = self.constants.get(&format!("{}{:02}", r, z) as &str) {
            bits.push(b);
            z += 1;
        }
        bits.reverse();
        let mut answer = 0;
        for z in bits {
            answer = answer << 1;
            if z {
                answer += 1;
            }
        }
        answer
    }
}

#[aoc(day24, part1)]
pub fn part1(input: &str) -> usize {
    let mut blocks = input.split("\n\n");
    let mut values = Registers::default();
    for c in blocks.next().unwrap().lines() {
        let mut x = c.split(": ");
        values.add(
            x.next().unwrap(),
            if x.next().unwrap() == "0" {
                false
            } else {
                true
            },
        );
    }
    for inst in blocks.next().unwrap().lines() {
        let toks: Vec<&str> = inst.split_ascii_whitespace().collect();
        values.add_gate(toks[1], toks[0], toks[2], toks[4]);
    }
    values.register('z')
}

#[aoc(day24, part2)]
pub fn part2(_input: &str) -> String {
    0
}
