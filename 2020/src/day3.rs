#[aoc_generator(day3)]
fn trees(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|l| l.chars().map(|t| t == '#').collect())
        .collect()
}

#[aoc(day3, part1)]
fn part1(landscape: &Vec<Vec<bool>>) -> usize {
    trees_hit(landscape, 3, 1)
}

fn trees_hit(landscape: &Vec<Vec<bool>>, dx: usize, dy: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;
    let width = landscape[0].len();
    loop {
        x = (x + dx) % width;
        y += dy;
        if y >= landscape.len() {
            break;
        }
        if landscape[y][x] {
            trees += 1;
        }
    }
    trees
}

#[aoc(day3, part2)]
fn part2(landscape: &Vec<Vec<bool>>) -> usize {
    trees_hit(landscape, 1, 1)
        * trees_hit(landscape, 3, 1)
        * trees_hit(landscape, 5, 1)
        * trees_hit(landscape, 7, 1)
        * trees_hit(landscape, 1, 2)
}
