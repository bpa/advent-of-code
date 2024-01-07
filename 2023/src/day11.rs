use std::cmp::Ordering;

fn solve(input: &str, expansion: usize) -> usize {
    let width = input.find('\n').unwrap();
    let mut galaxies = Vec::new();
    let mut x_distance = vec![0; width];
    let mut y_distance = Vec::new();
    y_distance.push(0);
    let mut x = 0;
    let mut y = 0;
    for c in input.bytes() {
        match c {
            b'#' => {
                galaxies.push((x, y));
                x_distance[x] += 1;
                y_distance[y] += 1;
                x += 1;
            }
            b'\n' => {
                x = 0;
                y += 1;
                y_distance.push(0);
            }
            _ => x += 1,
        }
    }

    x_distance[0] = if x_distance[0] == 0 { expansion } else { 1 };
    for x in 1..x_distance.len() {
        x_distance[x] = x_distance[x - 1] + if x_distance[x] == 0 { expansion } else { 1 };
    }
    y_distance[0] = if y_distance[0] == 0 { expansion } else { 1 };
    for y in 1..y_distance.len() {
        y_distance[y] = y_distance[y - 1] + if y_distance[y] == 0 { expansion } else { 1 };
    }

    let mut total = 0;
    for (i, (ax, ay)) in galaxies.iter().enumerate() {
        for (bx, by) in galaxies.iter().skip(i) {
            total += match ax.cmp(bx) {
                Ordering::Less => x_distance[*bx] - x_distance[*ax],
                Ordering::Equal => 0,
                Ordering::Greater => x_distance[*ax] - x_distance[*bx],
            };
            total += match ay.cmp(by) {
                Ordering::Less => y_distance[*by] - y_distance[*ay],
                Ordering::Equal => 0,
                Ordering::Greater => y_distance[*ay] - y_distance[*by],
            };
        }
    }
    total
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    solve(input, 2)
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> usize {
    solve(input, 1_000_000)
}
