use std::convert::From;
use std::fmt::Display;

fn knot_step(data: &Vec<usize>) -> [usize; 256] {
    let mut skip = 0;
    let mut pos = 0;
    let mut hash = [0; 256];
    for i in 0..256 {
        hash[i] = i;
    }
    for length in data {
        if *length > 1 {
            let l_diff = length - 1;
            for i in 0..(length / 2) {
                let a = (pos + i) % 256;
                let b = (pos + l_diff - i) % 256;
                let tmp = hash[a];
                hash[a] = hash[b];
                hash[b] = tmp;
            }
        }
        pos = (pos + length + skip) % 256;
        skip += 1;
    }
    hash
}

pub struct KnotHash(Vec<u8>);
const POSTLUDE: &str = "\x11\x1fI/\x17";

impl From<&str> for KnotHash {
    fn from(input: &str) -> Self {
        let mut hash = [0; 256];
        for i in 0..256 {
            hash[i] = i;
        }

        let mut skip = 0usize;
        let mut pos = 0usize;
        for _ in 0..64 {
            for length in input.bytes().chain(POSTLUDE.bytes()) {
                if length > 1 {
                    let l_diff = length as usize - 1;
                    for i in 0..(length as usize / 2) {
                        let a = (pos + i) % 256;
                        let b = (pos + l_diff - i) % 256;
                        let tmp = hash[a];
                        hash[a] = hash[b];
                        hash[b] = tmp;
                    }
                }
                pos = (pos + length as usize + skip) % 256;
                skip += 1;
            }
        }

        let knot: Vec<u8> = hash
            .chunks(16)
            .map(|chunk| chunk.iter().fold(0, |a, b| a ^ b))
            .map(|num| num as u8)
            .collect();
        KnotHash(knot)
    }
}

impl Display for KnotHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.0[5],
            self.0[6],
            self.0[7],
            self.0[8],
            self.0[9],
            self.0[10],
            self.0[11],
            self.0[12],
            self.0[13],
            self.0[14],
            self.0[15]
        )
    }
}

impl KnotHash {
    pub fn count_ones(&self) -> usize {
        self.0.iter().map(|e| e.count_ones() as usize).sum()
    }

    pub fn as_bool_slice(&self) -> [bool; 128] {
        let mut row = [false; 128];
        let mut ind = 0;
        for i in 0..16 {
            let mut mask = 0b10000000;
            for _ in 0..8 {
                if self.0[i] & mask > 0 {
                    row[ind] = true;
                }
                ind += 1;
                mask >>= 1;
            }
        }
        row
    }
}

#[aoc_generator(day10, part1)]
fn lengths(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|num| num.parse::<usize>().unwrap())
        .collect()
}

#[aoc(day10, part1)]
fn first_two(input: &Vec<usize>) -> usize {
    let hash = knot_step(input);
    hash[0] * hash[1]
}

#[aoc(day10, part2)]
fn full_hash(input: &str) -> KnotHash {
    KnotHash::from(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn happy() {
        assert_eq!([2, 1, 0, 3, 4], knot_step(&vec![3])[0..5]);
        assert_eq!([2, 1, 0, 4, 3], knot_step(&vec![3, 2])[0..5]);
        assert_eq!([3, 2, 1, 0, 4], knot_step(&vec![4])[0..5]);
        assert_eq!([4, 3, 2, 1, 0], knot_step(&vec![5])[0..5]);
    }

    #[test]
    fn pos_changing() {
        assert_eq!([1, 0, 2, 3, 4], knot_step(&vec![0, 2])[0..5]);
        assert_eq!([0, 1, 3, 2, 4], knot_step(&vec![0, 1, 2])[0..5]);
    }

    #[test]
    fn wrapping() {
        assert_eq!([255, 254, 251, 250, 249], knot_step(&vec![254, 4])[0..5]);
    }

    #[test]
    fn known_hashes() {
        assert_eq!(
            "a2582a3a0e66e6e86e3812dcb672a272",
            format!("{}", KnotHash::from(""))
        );
        assert_eq!(
            "33efeb34ea91902bb2f59c9920caa6cd",
            format!("{}", KnotHash::from("AoC 2017"))
        );
        assert_eq!(
            "3efbe78a8d82f29979031a4aa0b16a9d",
            format!("{}", KnotHash::from("1,2,3"))
        );
        assert_eq!(
            "63960835bcdc130f0b66d7ff4f6a5a8e",
            format!("{}", KnotHash::from("1,2,4"))
        );
    }
}
