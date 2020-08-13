use cpu::{Input, Intcode};
use std::iter::once;
use std::num::ParseIntError;

#[aoc_generator(day7)]
fn read_memory(input: &str) -> Result<Vec<isize>, ParseIntError> {
    input.split(',').map(|num| num.parse::<isize>()).collect()
}

#[aoc(day7, part1)]
pub fn max_thrust(m: &Vec<isize>) -> String {
    let answer = Phase::new((0..5).collect())
        .map(|p| {
            let mut output: isize = 0;
            for phase in p {
                output = Intcode::new(m.clone(), &mut vec![phase as isize, output].into_iter())
                    .last()
                    .unwrap();
            }
            output
        })
        .max()
        .unwrap();
    format!("{:?}", answer)
}

#[aoc(day7, part2)]
pub fn max_thrust_with_feedback(m: &Vec<isize>) -> String {
    let answer = Phase::new((5..10).collect())
        .map(|p| output_value(m, &p))
        .max()
        .unwrap();
    format!("{:?}", answer)
}

fn output_value(m: &Vec<isize>, phase: &[isize]) -> isize {
    macro_rules! computer {
        ( $name:ident = $phase:literal >> $input:ident ) => {
            let mut chain = once(phase[$phase]).chain(&mut $input);
            let mut $name = Intcode::new(m.clone(), &mut chain);
        };
    }
    let input: Input = Input::new(&[phase[0], 0]);
    let mut one = Intcode::new(m.clone(), input.iter_mut());
    computer!(two = 1 >> one);
    computer!(three = 2 >> two);
    computer!(four = 3 >> three);
    computer!(five = 4 >> four);
    let mut output: isize = 0;
    loop {
        match five.next() {
            Some(value) => {
                output = value;
                input.push(output);
            }
            None => return output,
        }
    }
}

#[derive(Debug)]
struct Phase {
    n: usize,
    curr: Vec<isize>,
    state: Vec<usize>,
    i: usize,
}

impl Phase {
    fn new(items: Vec<isize>) -> Self {
        let size = items.len();
        Phase {
            n: size,
            curr: items,
            state: (0..size).map(|_| 0).collect(),
            i: 0,
        }
    }
}

impl Iterator for Phase {
    type Item = Vec<isize>;

    fn next(&mut self) -> Option<Vec<isize>> {
        if self.i >= self.n {
            return None;
        }

        let permutation = self.curr.clone();
        while self.i < self.n {
            if self.state[self.i] < self.i {
                if self.i % 2 == 0 {
                    let tmp = self.curr[0];
                    self.curr[0] = self.curr[self.i];
                    self.curr[self.i] = tmp;
                } else {
                    let tmp = self.curr[self.state[self.i]];
                    self.curr[self.state[self.i]] = self.curr[self.i];
                    self.curr[self.i] = tmp;
                }
                self.state[self.i] = self.state[self.i] + 1;
                self.i = 0;
                return Some(permutation);
            } else {
                self.state[self.i] = 0;
                self.i = self.i + 1;
            }
        }

        Some(permutation)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn all_permutations() {
        let mut p = Phase::new((0..3).collect());
        assert_eq!(Some(vec!(0, 1, 2)), p.next());
        assert_eq!(Some(vec!(1, 0, 2)), p.next());
        assert_eq!(Some(vec!(2, 0, 1)), p.next());
        assert_eq!(Some(vec!(0, 2, 1)), p.next());
        assert_eq!(Some(vec!(1, 2, 0)), p.next());
        assert_eq!(Some(vec!(2, 1, 0)), p.next());
        assert_eq!(None, p.next());
    }
}
