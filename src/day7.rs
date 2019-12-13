use cpu::run;
use std::num::ParseIntError;

#[aoc_generator(day7)]
fn read_memory(input: &str) -> Result<Vec<isize>, ParseIntError> {
    input.split(',').map(|num| num.parse::<isize>()).collect()
}

#[aoc(day7, part1)]
pub fn run_system_1(m: &Vec<isize>) -> String {
    let answer = Phase::new(5)
        .map(|p| {
            let mut output: isize = 0;
            for phase in p {
                let code = m.clone();
                run(code, vec![phase as isize, output], &mut |_, val| {
                    output = val
                });
            }
            output
        })
        .max()
        .unwrap();
    format!("{:?}", answer)
}

#[derive(Debug)]
struct Phase {
    n: usize,
    curr: Vec<usize>,
    state: Vec<usize>,
    i: usize,
}

impl Phase {
    fn new(count: usize) -> Self {
        Phase {
            n: count,
            curr: (0..count).collect(),
            state: (0..count).map(|_| 0).collect(),
            i: 0,
        }
    }
}

impl Iterator for Phase {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Vec<usize>> {
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
        let mut p = Phase::new(3);
        assert_eq!(Some(vec!(0, 1, 2)), p.next());
        assert_eq!(Some(vec!(1, 0, 2)), p.next());
        assert_eq!(Some(vec!(2, 0, 1)), p.next());
        assert_eq!(Some(vec!(0, 2, 1)), p.next());
        assert_eq!(Some(vec!(1, 2, 0)), p.next());
        assert_eq!(Some(vec!(2, 1, 0)), p.next());
        assert_eq!(None, p.next());
    }
}
