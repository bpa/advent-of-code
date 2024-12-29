use aoc::*;

fn is_wall(s: bool) -> CellType {
    match s {
        true => CellType::Occupied,
        false => CellType::Empty,
    }
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> usize {
    let mut grid = Grid::of(71, 71, false);
    for mut toks in input.lines().take(1024).map(|l| l.split(",")) {
        grid.set(
            toks.next().unwrap().parse::<usize>().unwrap(),
            toks.next().unwrap().parse::<usize>().unwrap(),
            true,
        );
    }
    grid.shortest_path(0, 0, 70, 70, &is_wall).unwrap().len()
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> String {
    let mut grid = Grid::of(71, 71, false);
    for mut toks in input.lines().take(1024).map(|l| l.split(",")) {
        grid.set(
            toks.next().unwrap().parse::<usize>().unwrap(),
            toks.next().unwrap().parse::<usize>().unwrap(),
            true,
        );
    }
    for mut toks in input.lines().skip(1024).map(|l| l.split(",")) {
        let x = toks.next().unwrap().parse::<usize>().unwrap();
        let y = toks.next().unwrap().parse::<usize>().unwrap();
        grid.set(x, y, true);
        if grid.shortest_path(0, 0, 70, 70, &is_wall).is_none() {
            return format!("{},{}", x, y);
        }
    }
    String::from("NaN")
}
