use aoc::*;

fn find_xmas(p: Point2D<u8>) -> usize {
    if p.get() != b'X' {
        return 0;
    }
    let mut count = 0;
    for x in -1..=1 {
        'nf: for y in -1..=1 {
            let mut curr = p.clone();
            for c in "MAS".bytes() {
                match curr.neighbor(x, y) {
                    Some(n) => {
                        if c != n.get() {
                            continue 'nf;
                        }
                        curr = n;
                    }
                    None => continue 'nf,
                }
            }
            count += 1;
        }
    }
    count
}

fn find_x_mas(p: Point2D<u8>) -> bool {
    p.get() == b'A' && is_cross(p.ne(), p.sw()) && is_cross(p.nw(), p.se())
}

fn get_or_x(a: Option<Point2D<u8>>) -> u8 {
    match a {
        Some(v) => v.get(),
        None => b'X',
    }
}

fn is_cross(a: Option<Point2D<u8>>, b: Option<Point2D<u8>>) -> bool {
    match (get_or_x(a), get_or_x(b)) {
        (b'M', b'S') | (b'S', b'M') => true,
        (_, _) => false,
    }
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    let puzzle: Grid<u8> = Grid::from(input);
    puzzle.points().map(|p| find_xmas(p)).sum()
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    let puzzle: Grid<u8> = Grid::from(input);
    puzzle.points().filter(|p| find_x_mas(p.clone())).count()
}
