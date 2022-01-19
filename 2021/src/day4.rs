use std::collections::HashSet;

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    let (numbers, mut boards) = parse(input);

    for n in numbers {
        for b in &mut boards {
            if b.mark(n) {
                return n * b.sum();
            }
        }
    }

    0
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    let (numbers, mut boards) = parse(input);

    for n in numbers {
        let mut finished = None;
        for (i, b) in boards.iter_mut().enumerate() {
            if !b.1 && b.mark(n) {
                finished = Some(i);
            }
        }
        if let Some(i) = finished {
            if boards.iter().all(|b| b.1) {
                return n * boards[i].sum();
            }
        }
    }

    0
}

fn parse(input: &str) -> (Vec<usize>, Vec<Board>) {
    let mut lines = input.lines();

    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let lines: Vec<&str> = lines.collect();
    let boards = lines
        .chunks(6)
        .map(|y| {
            y.iter()
                .skip(1)
                .map(|z| {
                    z.split_ascii_whitespace()
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>()
        })
        .map(|b| Board::new(b))
        .collect::<Vec<Board>>();
    (numbers, boards)
}

#[derive(Debug)]
struct Board(Vec<HashSet<usize>>, bool);

impl Board {
    fn new(numbers: Vec<Vec<usize>>) -> Self {
        let mut board = Board(vec![HashSet::new(); 10], false);
        for x in 0..5 {
            for y in 0..5 {
                board.0[x].insert(numbers[x][y]);
                board.0[x + 5].insert(numbers[y][x]);
            }
        }
        board
    }

    fn mark(&mut self, n: usize) -> bool {
        for b in &mut self.0 {
            b.remove(&n);
        }
        if self.0.iter().any(|l| l.is_empty()) {
            self.1 = true;
        }
        self.1
    }

    fn sum(&self) -> usize {
        self.0
            .iter()
            .flat_map(|l| l.iter())
            .copied()
            .collect::<HashSet<usize>>()
            .iter()
            .sum()
    }
}
