use std::cmp::{max, min, Ordering};

#[aoc_generator(day6)]
fn points(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let nums: Vec<usize> = line
                .split(", ")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            return (nums[0], nums[1]);
        })
        .collect()
}

#[aoc(day6, part1)]
fn loneliest(input: &Vec<(usize, usize)>) -> usize {
    let (min_x, min_y, max_x, max_y) = bounds(input);
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    let mut area = vec![0; input.len() + 1];
    let mut owner = vec![vec![input.len(); height]; width];
    let mut distance = vec![vec![usize::MAX; height]; width];
    area[input.len()] = width * height;

    for (i, &(x, y)) in input.iter().enumerate() {
        let x1 = x - min_x;
        let y1 = y - min_y;
        for y0 in 0..height {
            for x0 in 0..width {
                let d = (x0 as isize - x1 as isize).abs() + (y0 as isize - y1 as isize).abs();
                match distance[x0][y0].cmp(&(d as usize)) {
                    Ordering::Less => {}
                    Ordering::Greater => {
                        let prev_owner = owner[x0][y0];
                        owner[x0][y0] = i;
                        distance[x0][y0] = d as usize;
                        area[prev_owner] -= 1;
                        area[i] += 1;
                    }
                    Ordering::Equal => {
                        let prev_owner = owner[x0][y0];
                        owner[x0][y0] = input.len();
                        area[prev_owner] -= 1;
                        area[input.len()] += 1;
                    }
                }
            }
        }
    }

    for x in 0..width {
        area[owner[x][0]] = 0;
        area[owner[x][height - 1]] = 0;
    }
    for y in 0..height {
        area[owner[0][y]] = 0;
        area[owner[width - 1][y]] = 0;
    }
    area[input.len()] = 0;
    *area.iter().max().unwrap()
}

fn bounds(input: &Vec<(usize, usize)>) -> (usize, usize, usize, usize) {
    let mut min_x = usize::MAX;
    let mut min_y = usize::MAX;
    let mut max_x = 0;
    let mut max_y = 0;

    for &(x, y) in input {
        min_x = min(x, min_x);
        min_y = min(y, min_y);
        max_x = max(x, max_x);
        max_y = max(y, max_y);
    }
    (min_x, min_y, max_x, max_y)
}

#[aoc(day6, part2)]
fn central(input: &Vec<(usize, usize)>) -> usize {
    let (min_x, min_y, max_x, max_y) = bounds(input);
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    let mut distance = vec![0; height * width];

    for &(x, y) in input {
        let x1 = x - min_x;
        let y1 = y - min_y;
        for y0 in 0..height {
            for x0 in 0..width {
                distance[x0 + y0 * width] +=
                    (x0 as isize - x1 as isize).abs() + (y0 as isize - y1 as isize).abs();
            }
        }
    }

    distance.iter().filter(|&d| d < &10_000).count()
}
