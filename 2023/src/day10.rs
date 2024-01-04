use aoc::*;

const FLOW: [[usize; 4]; 6] = [
    [0, 9, 2, 9], // |
    [9, 1, 9, 3], // -
    [9, 9, 1, 0], // L
    [9, 0, 3, 9], // J
    [3, 2, 9, 9], // 7
    [1, 9, 9, 2], // F
];

#[aoc_generator(day10)]
pub fn parse(input: &str) -> Grid<usize> {
    Grid::parse_str(input, |p| match p {
        b'|' => 0,
        b'-' => 1,
        b'L' => 2,
        b'J' => 3,
        b'7' => 4,
        b'F' => 5,
        v => v as usize,
    })
}

const START_PIPE: [[usize; 3]; 4] = [[0, 4, 5], [1, 3, 4], [0, 2, 3], [1, 2, 5]];
fn start(p: Point2D<usize>) -> (usize, Point2D<usize>) {
    for (d, &(dx, dy)) in CARDINAL_4.into_iter().enumerate() {
        if let Some(n) = p.neighbor(dx, dy) {
            let pipe = n.get();
            if START_PIPE[d].iter().any(|&s| s == pipe) {
                return (d, n);
            }
        }
    }
    panic!("Invalid pipe maze")
}

#[aoc(day10, part1)]
pub fn part1(input: &Grid<usize>) -> usize {
    let (mut d, p) = start(input.index_of(b'S' as usize).unwrap());
    let pipes = input.data();
    let mut x = p.x;
    let mut y = p.y;
    let mut steps = 1;
    let mut p = pipes[y][x];
    while p != b'S' as usize {
        steps += 1;
        d = FLOW[p][d];
        let (dx, dy) = CARDINAL_4[d];
        x = (x as isize + dx) as usize;
        y = (y as isize + dy) as usize;
        p = pipes[y][x];
    }
    steps / 2
}

#[aoc(day10, part2)]
pub fn part2(input: &Grid<usize>) -> isize {
    let p = input.index_of(b'S' as usize).unwrap();
    let mut x0 = p.x;
    let mut y0 = p.y;
    let (mut d, p) = start(p);
    let pipes = input.data();
    let mut x = p.x;
    let mut y = p.y;
    let mut area = 0;
    let mut p = pipes[y][x];
    let mut steps = 1;
    loop {
        if (p >= 2 && p <= 5) || p == b'S' as usize {
            area += (x0 * y) as isize;
            area -= (x * y0) as isize;
            x0 = x;
            y0 = y;

            if p == b'S' as usize {
                return (area.abs() - steps) / 2 + 1;
            }
        }
        steps += 1;
        d = FLOW[p][d];
        let (dx, dy) = CARDINAL_4[d];
        x = (x as isize + dx) as usize;
        y = (y as isize + dy) as usize;
        p = pipes[y][x];
    }
}
