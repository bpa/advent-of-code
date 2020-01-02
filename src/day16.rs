use std::char;

const PATTERN: [isize; 4] = [0, 1, 0, -1];

#[aoc_generator(day16)]
fn read_memory(input: &str) -> Vec<isize> {
    input
        .chars()
        .map(|num| num.to_digit(10).unwrap() as isize)
        .collect()
}

#[aoc(day16, part1)]
pub fn eight_digits(input: &[isize]) -> String {
    let mut seq = Vec::from(input);
    for _ in 0..100 {
        seq = (1..=seq.len())
            .map(|i| {
                let mut phase = Phase::new(&PATTERN, i);
                phase.next();
                (seq.iter().zip(phase).map(|(a, b)| a * b).sum::<isize>() % 10).abs()
            })
            .collect();
    }
    seq[0..8]
        .iter()
        .map(|c| char::from_digit(c.abs() as u32, 10).unwrap())
        .collect()
}

struct Phase<'a> {
    sequence: &'a [isize],
    repeat: usize,
    i: usize,
    step: usize,
}

impl<'a> Phase<'a> {
    fn new(seq: &'a [isize], repeat: usize) -> Self {
        Phase {
            sequence: seq,
            repeat: repeat,
            i: 0,
            step: 0,
        }
    }
}

impl<'a> Iterator for Phase<'a> {
    type Item = isize;

    fn next(&mut self) -> Option<isize> {
        if self.step == self.repeat {
            self.step = 1;
            self.i = self.i + 1;
            if self.i == self.sequence.len() {
                self.i = 0;
            }
        } else {
            self.step = self.step + 1;
        }
        Some(self.sequence[self.i])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iterator_1() {
        let seq = vec![1, 2, 3];
        let mut phase = Phase::new(&seq, 1);
        assert_eq!(Some(1), phase.next());
        assert_eq!(Some(2), phase.next());
        assert_eq!(Some(3), phase.next());
        assert_eq!(Some(1), phase.next());
    }

    #[test]
    fn iterator_3() {
        let seq = vec![1, 2, 3];
        let mut phase = Phase::new(&seq, 3);
        assert_eq!(Some(1), phase.next());
        assert_eq!(Some(1), phase.next());
        assert_eq!(Some(1), phase.next());
        assert_eq!(Some(2), phase.next());
    }
}
