use crate::day10::KnotHash;
use std::collections::VecDeque;

#[aoc(day14, part1)]
fn squares_used(input: &str) -> usize {
    (0..128)
        .map(|row| KnotHash::from(format!("{}-{}", input, row).as_str()))
        .map(|hash| hash.count_ones())
        .sum()
}

#[aoc(day14, part2)]
fn regions(input: &str) -> usize {
    let grid: Vec<[bool; 128]> = (0..128)
        .map(|row| KnotHash::from(format!("{}-{}", input, row).as_str()))
        .map(|hash| hash.as_bool_slice())
        .collect();
    let mut visited: Vec<[bool; 128]> = (0..128).map(|_| [false; 128]).collect();

    let mut regions = 0;
    for y in 0..128 {
        for x in 0..128 {
            if !visited[y][x] {
                if grid[y][x] {
                    regions += 1;
                    visit_region(x, y, &grid, &mut visited);
                } else {
                    visited[y][x] = true;
                }
            }
        }
    }
    regions
}

fn visit_region(x: usize, y: usize, grid: &Vec<[bool; 128]>, visited: &mut Vec<[bool; 128]>) {
    let mut to_visit = VecDeque::new();
    to_visit.push_back((x, y));
    while let Some((col, row)) = to_visit.pop_back() {
        visited[row][col] = true;
        if grid[row][col] {
            if col > 0 && !visited[row][col - 1] {
                to_visit.push_back((col - 1, row));
            }
            if col < 127 && !visited[row][col + 1] {
                to_visit.push_back((col + 1, row));
            }
            if row > 0 && !visited[row - 1][col] {
                to_visit.push_back((col, row - 1));
            }
            if row < 127 && !visited[row + 1][col] {
                to_visit.push_back((col, row + 1));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(8108, squares_used("flqrgnkx"));
    }

    #[test]
    fn part2() {
        assert_eq!(1242, regions("flqrgnkx"));
    }
}
