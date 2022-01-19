use std::cmp::Ordering;

#[aoc(day5, part1)]
fn d5part1(input: &str) -> isize {
    let vectors = input
        .lines()
        .map(|l| l.split(" -> "))
        .map(|mut l| {
            let a = l.next().unwrap();
            let b = l.next().unwrap();
            let mut one = a.split(',');
            let x = one.next().unwrap().parse::<isize>().unwrap();
            let y = one.next().unwrap().parse::<isize>().unwrap();
            let mut two = b.split(',');
            let xx = two.next().unwrap().parse::<isize>().unwrap();
            let yy = two.next().unwrap().parse::<isize>().unwrap();
            (x, y, xx, yy)
        })
        .collect::<Vec<(isize, isize, isize, isize)>>();
    let (max_x, max_y) = vectors
        .iter()
        .map(|(a, b, c, d)| (a.max(c), b.max(d)))
        .reduce(|a, b| (a.0.max(b.0), a.1.max(b.1)))
        .unwrap();
    let mut area = vec![vec![0; *max_y as usize + 1]; *max_x as usize + 1];
    for (x0, y0, x1, y1) in vectors {
        let dx = match x0.cmp(&x1) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        let dy = match y0.cmp(&y1) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        let mut x = x0;
        let mut y = y0;
        area[y as usize][x as usize] += 1;
        loop {
            x += dx;
            y += dy;
            area[y as usize][x as usize] += 1;
            if x == x1 && y == y1 {
                break;
            }
        }
    }
    area.iter()
        .map(|c| c.iter().fold(0, |c, r| if *r > 1 { c + 1 } else { c }))
        .sum()
}

#[aoc(day5, part2)]
fn d5part2(_input: &str) -> isize {
    0
}
